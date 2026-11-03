#!/usr/bin/env python3
"""Safe Ubuntu 22.04 CI smoke checks. No exploit, real credentials or external traffic."""
import ctypes
import errno
import json
import os
import pathlib
import platform
import pwd
import shutil
import socket
import subprocess
import sys
import tempfile

def verify_dac():
    if os.geteuid() != 0:
        raise RuntimeError("Run this disposable-host test with sudo")
    u = pwd.getpwnam("nobody")
    with tempfile.TemporaryDirectory(prefix="pelaos-jammy-") as tmp:
        base = pathlib.Path(tmp)
        os.chmod(base, 0o755)
        work, locked, outside = (base / x for x in ("work", "locked", "outside"))
        for d in (work, locked, outside):
            d.mkdir()
        os.chown(work, u.pw_uid, u.pw_gid)
        work.chmod(0o700)
        locked.chmod(0o700)
        outside.chmod(0o755)
        (locked / "dummy_canary.txt").write_text("NOT A SECRET\n")
        (locked / "dummy_canary.txt").chmod(0o600)
        child = '''
import json, os, pathlib, sys
base = pathlib.Path(sys.argv[1])
ops = {
 "canary_read": lambda: (base/"locked"/"dummy_canary.txt").read_text(),
 "workspace_write": lambda: (base/"work"/"ok.txt").write_text("ok"),
 "outside_write": lambda: (base/"outside"/"blocked.txt").write_text("no"),
}
out = {}
for key, op in ops.items():
 try:
  op()
  out[key] = "ALLOWED"
 except PermissionError:
  out[key] = "DENIED"
out["euid"] = os.geteuid()
out["no_new_privs"] = next((s.split(":", 1)[1].strip() for s in pathlib.Path("/proc/self/status").read_text().splitlines() if s.startswith("NoNewPrivs:")), "unknown")
print(json.dumps(out))
'''
        cp = subprocess.run(["setpriv", "--reuid=" + str(u.pw_uid), "--regid=" + str(u.pw_gid), "--clear-groups", "--no-new-privs", sys.executable, "-c", child, str(base)], text=True, capture_output=True, timeout=10)
        if cp.returncode:
            raise RuntimeError(cp.stderr[-500:])
        result = json.loads(cp.stdout)
        expected = {"canary_read": "DENIED", "workspace_write": "ALLOWED", "outside_write": "DENIED", "euid": u.pw_uid, "no_new_privs": "1"}
        return {"pass": all(result.get(k) == v for k,v in expected.items()), "observed": result}

def verify_seccomp():
    # Positive/negative control: localhost only. No public network is contacted.
    child = '''
import ctypes, errno, json, socket, sys
port = int(sys.argv[1])
if sys.argv[2] == "deny":
 lib = ctypes.CDLL("libseccomp.so.2")
 lib.seccomp_init.argtypes = [ctypes.c_uint32]
 lib.seccomp_init.restype = ctypes.c_void_p
 lib.seccomp_syscall_resolve_name.argtypes = [ctypes.c_char_p]
 lib.seccomp_syscall_resolve_name.restype = ctypes.c_int
 lib.seccomp_rule_add.argtypes = [ctypes.c_void_p, ctypes.c_uint32, ctypes.c_int, ctypes.c_uint]
 lib.seccomp_rule_add.restype = ctypes.c_int
 lib.seccomp_load.argtypes = [ctypes.c_void_p]
 lib.seccomp_load.restype = ctypes.c_int
 lib.seccomp_release.argtypes = [ctypes.c_void_p]
 ctx = lib.seccomp_init(0x7fff0000)
 syscall = lib.seccomp_syscall_resolve_name(b"connect")
 assert ctx and syscall >= 0
 assert lib.seccomp_rule_add(ctx, 0x00050000 | errno.EPERM, syscall, 0) == 0
 assert lib.seccomp_load(ctx) == 0
 lib.seccomp_release(ctx)
with socket.socket() as s:
 s.settimeout(2)
 try:
  s.connect(("127.0.0.1", port))
  observed = "CONNECTED"
 except OSError as e:
  observed = "DENIED_EPERM" if e.errno == errno.EPERM else "ERROR:" + repr(e)
print(json.dumps({"outcome": observed}))
'''
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        listener.listen(4)
        port = listener.getsockname()[1]
        result = {}
        for mode in ("baseline", "deny"):
            cp = subprocess.run([sys.executable, "-c", child, str(port), mode], capture_output=True, text=True, timeout=10)
            if cp.returncode:
                raise RuntimeError(mode + ": " + cp.stderr[-600:])
            result[mode] = json.loads(cp.stdout)
        return {"pass": result["baseline"]["outcome"] == "CONNECTED" and result["deny"]["outcome"] == "DENIED_EPERM", "observed": result}

def main():
    info={}
    for line in pathlib.Path("/etc/os-release").read_text().splitlines():
        if "=" in line:
            k,v=line.split("=",1)
            info[k]=v.strip('"')
    report={"os": info.get("PRETTY_NAME"), "version": info.get("VERSION_ID"), "kernel": platform.release(), "runner_note": "GitHub-hosted Jammy runner kernel may not match stock Ubuntu 22.04 GA", "scope": "DAC and kernel seccomp localhost checks only, no AppArmor policy or Snap execution, no exploit", "tests": {}, "inventory": {}}
    for name,fn in (("dac",verify_dac),("seccomp",verify_seccomp)):
        try:
            report["tests"][name]=fn()
        except Exception as e:
            report["tests"][name]={"pass": False, "error": repr(e)}
    for tool in ("aa-status","snap"):
        executable=shutil.which(tool)
        if executable:
            args=[executable] + (["version"] if tool=="snap" else [])
            cp=subprocess.run(args,capture_output=True,text=True,timeout=10)
            report["inventory"][tool]={"return_code": cp.returncode, "output": (cp.stdout+cp.stderr).strip()[:700], "note": "inventory only"}
        else:
            report["inventory"][tool]={"available": False}
    report["pass"]=report["version"]=="22.04" and all(x["pass"] for x in report["tests"].values())
    print(json.dumps(report,indent=2))
    return 0 if report["pass"] else 1

if __name__ == "__main__":
    raise SystemExit(main())
