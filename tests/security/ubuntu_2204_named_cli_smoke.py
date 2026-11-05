#!/usr/bin/env python3
"""Run pinned named CLI --version with synthetic, same-UID AppArmor fixtures.

Only disposable Ubuntu 22.04 CI. This is NOT an agent prompt/tool-use test,
not a complete sandbox and not a production-ready AppArmor policy.
SPDX-License-Identifier: MIT
"""
from __future__ import annotations

import argparse
import json
import os
from pathlib import Path
import platform
import pwd
import shutil
import subprocess
import sys
import tempfile


CHILD = r"""
import ctypes
import errno
import json
import os
from pathlib import Path
import socket
import subprocess
import sys

root = Path(sys.argv[1])
cli = Path(sys.argv[2])
version = sys.argv[3]

def observe(action):
    try:
        action()
        return "ALLOWED"
    except PermissionError:
        return "DENIED"
    except OSError as exc:
        return "ERRNO_" + str(exc.errno)

def filter_network():
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
    context = lib.seccomp_init(0x7fff0000)
    if not context:
        raise RuntimeError("seccomp_init failed")
    try:
        for name in (b"socket", b"socketpair", b"connect"):
            number = lib.seccomp_syscall_resolve_name(name)
            if number < 0 or lib.seccomp_rule_add(
                context, 0x00050000 | errno.EPERM, number, 0
            ):
                raise RuntimeError("Failed to add network rule: " + repr(name))
        if lib.seccomp_load(context):
            raise RuntimeError("seccomp_load failed")
    finally:
        lib.seccomp_release(context)

def socket_probe():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM):
        pass

def shell_probe():
    try:
        completed = subprocess.run(["/bin/sh", "-c", ":"], capture_output=True,
                                   timeout=4, check=False)
        return "ALLOWED" if completed.returncode == 0 else "EXIT_" + str(completed.returncode)
    except PermissionError:
        return "DENIED"

# Positive syscall check occurs before the filter; never connects to any host.
socket_before = observe(socket_probe)
filter_network()
socket_after = observe(socket_probe)

p = subprocess.run([str(cli), "--version"], capture_output=True, text=True,
                   cwd=root / "work", timeout=30, check=False)
text = (p.stdout + p.stderr).strip()
result = {
    "uid": os.geteuid(),
    "label": Path("/proc/self/attr/current").read_text().strip(),
    "cli_exit": p.returncode,
    "cli_version_matches": p.returncode == 0 and version in text,
    "cli_output_excerpt": text[:180],
    "canary_direct_read": observe(lambda: (root / "protected" / "synthetic.txt").read_text()),
    "canary_symlink_read": observe(lambda: (root / "work" / "canary-link").read_text()),
    "outside_direct_write": observe(lambda: (root / "outside" / "target.txt").write_text("benign")),
    "outside_symlink_write": observe(lambda: (root / "work" / "outside-link").write_text("benign")),
    "workspace_write": observe(lambda: (root / "work" / "approved.txt").write_text("benign")),
    "shell_exec": shell_probe(),
    "socket_before_seccomp": socket_before,
    "socket_after_seccomp": socket_after,
}
print(json.dumps(result))
"""


def run(command: list[str], *, env=None, timeout=40):
    return subprocess.run(command, env=env, text=True, capture_output=True,
                          timeout=timeout, check=False)


def probe(prefix: list[str], root: Path, cli: Path, version: str,
          node_bin: Path) -> dict:
    # No real user's home, GitHub token, private SSH keys or provider credentials.
    clean_env = {
        "PATH": str(node_bin.parent) + ":/usr/bin:/bin",
        "HOME": str(root / "work"),
        "XDG_CONFIG_HOME": str(root / "work" / "config"),
        "XDG_CACHE_HOME": str(root / "work" / "cache"),
        "XDG_DATA_HOME": str(root / "work" / "data"),
        "LANG": "C.UTF-8",
        "NO_COLOR": "1",
        "CI": "true",
        "DISABLE_TELEMETRY": "true",
        "DO_NOT_TRACK": "1",
    }
    proc = run(
        [*prefix, sys.executable, "-c", CHILD, str(root), str(cli), version],
        env=clean_env
    )
    if proc.returncode:
        raise RuntimeError("Fixture probe failed: " + proc.stderr[-700:])
    try:
        return json.loads(proc.stdout)
    except json.JSONDecodeError as exc:
        raise RuntimeError("Fixture returned invalid JSON: " + proc.stdout[-400:]) from exc


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--agent", choices=("claude", "codex"), required=True)
    parser.add_argument("--version", required=True)
    parser.add_argument("--npm-prefix", required=True, type=Path)
    parser.add_argument("--node-bin", required=True, type=Path)
    args = parser.parse_args()
    report = {
        "suite": "Named coding CLI version plus same-UID AppArmor file fixtures",
        "agent": args.agent,
        "expected_version": args.version,
        "kernel": platform.release(),
        "limitations": (
            "CLI --version only; no login, LLM prompts, model tool execution, "
            "provider APIs, real secrets, production policy, network egress proof, "
            "native Jammy kernel or Snap validation"
        ),
        "status": "FAIL",
    }

    try:
        if os.geteuid() != 0:
            raise RuntimeError("Root required solely for disposable VM profile load/unload")
        facts = dict(line.split("=", 1) for line in
                     Path("/etc/os-release").read_text().splitlines() if "=" in line)
        if facts.get("ID", "").strip('"') != "ubuntu" or facts.get(
            "VERSION_ID", ""
        ).strip('"') != "22.04":
            raise RuntimeError("Test environment is not Ubuntu 22.04")
        if any(shutil.which(tool) is None for tool in
               ("setpriv", "aa-exec", "apparmor_parser")):
            raise RuntimeError("Missing AppArmor or privilege utility")
        if not args.node_bin.resolve().is_file():
            raise RuntimeError("Pinned node runtime not found")
        if not (args.npm_prefix / "node_modules").is_dir():
            raise RuntimeError("Pinned npm installation missing")

        user = pwd.getpwnam("nobody")
        # Codex intentionally rejects helper creation inside the system /tmp.
        # Place disposable fixtures in the GitHub-hosted runner home instead.
        runner_home = Path("/home/runner")
        if not runner_home.is_dir():
            raise RuntimeError("Missing disposable GitHub runner home")
        with tempfile.TemporaryDirectory(
            prefix="pelaos-agent-cli-", dir=runner_home
        ) as temp:
            root = Path(temp)
            root.chmod(0o755)
            for section in ("work", "protected", "outside"):
                folder = root / section
                folder.mkdir(mode=0o700)
                os.chown(folder, user.pw_uid, user.pw_gid)
            for name, folder in (("synthetic.txt", "protected"), ("target.txt", "outside")):
                file = root / folder / name
                file.write_text("SYNTHETIC-DO-NOT-USE-AS-CREDENTIAL", encoding="utf-8")
                os.chown(file, user.pw_uid, user.pw_gid)
                file.chmod(0o600)
            canary = root / "protected" / "synthetic.txt"
            (root / "work" / "canary-link").symlink_to(canary)
            (root / "work" / "outside-link").symlink_to(root / "outside" / "target.txt")

            application = root / "application"
            shutil.copytree(args.npm_prefix, application, symlinks=True)
            cli = application / "node_modules" / ".bin" / args.agent
            if not cli.exists() or not os.access(cli, os.X_OK):
                raise RuntimeError("Pinned CLI launcher not executable")
            prefix = [
                "setpriv", "--reuid=" + str(user.pw_uid),
                "--regid=" + str(user.pw_gid), "--clear-groups",
                "--no-new-privs",
            ]
            baseline = probe(prefix, root, cli, args.version, args.node_bin)
            report["baseline"] = baseline
            required_baseline = {
                "uid": user.pw_uid,
                "label": "unconfined",
                "cli_version_matches": True,
                "canary_direct_read": "ALLOWED",
                "canary_symlink_read": "ALLOWED",
                "outside_direct_write": "ALLOWED",
                "outside_symlink_write": "ALLOWED",
                "workspace_write": "ALLOWED",
                "shell_exec": "ALLOWED",
                "socket_before_seccomp": "ALLOWED",
                "socket_after_seccomp": "DENIED",
            }
            if any(baseline.get(k) != v for k, v in required_baseline.items()):
                raise RuntimeError("Same-UID unconfined controls failed; see baseline")

            profile_name = "pelaos-cli-" + args.agent + "-" + str(os.getpid())
            node_path = args.node_bin.resolve()
            # Deliberate fixture allowances to boot Node/native CLIs. This
            # policy must NOT be reused as a least-privilege production policy.
            profile = f"""#include <tunables/global>
profile {profile_name} flags=(attach_disconnected) {{
  #include <abstractions/base>
  /usr/bin/python3* ix,
  /usr/bin/env ix,
  /usr/** mr,
  /lib/** mr,
  /lib64/** mr,
  /etc/** r,
  /proc/** r,
  /sys/** r,
  /dev/** rw,
  /run/** r,
  /opt/** mr,
  /home/ r,
  /home/runner/ r,
  {node_path} ix,
  {root}/ r,
  {application}/ r,
  {application}/** rm,
  {application}/** ix,
  {root}/work/ rw,
  {root}/work/** rwk,
  {root}/protected/ r,
  {canary} r,
  {root}/outside/ r,
  {root}/outside/target.txt rw,
  deny {canary} r,
  deny {root}/outside/** w,
}}
"""
            policy = root / "fixture.apparmor"
            policy.write_text(profile)
            loaded = False
            try:
                proc = run(["apparmor_parser", "-a", str(policy)])
                if proc.returncode:
                    raise RuntimeError("Failed to load AppArmor: " + proc.stderr[-650:])
                loaded = True
                confined = probe([*prefix, "aa-exec", "-p", profile_name, "--"],
                                 root, cli, args.version, args.node_bin)
                report["confined"] = confined
                expected = {
                    "uid": user.pw_uid,
                    "label": profile_name + " (enforce)",
                    "cli_version_matches": True,
                    "canary_direct_read": "DENIED",
                    "canary_symlink_read": "DENIED",
                    "outside_direct_write": "DENIED",
                    "outside_symlink_write": "DENIED",
                    "workspace_write": "ALLOWED",
                    "shell_exec": "DENIED",
                    "socket_before_seccomp": "ALLOWED",
                    "socket_after_seccomp": "DENIED",
                }
                report["assertions"] = {
                    name: confined.get(name) == value
                    for name, value in expected.items()
                }
                report["status"] = (
                    "PASS" if all(report["assertions"].values()) else "FAIL"
                )
            finally:
                if loaded:
                    removed = run(["apparmor_parser", "-R", str(policy)])
                    report["profile_unloaded"] = removed.returncode == 0
                    if removed.returncode:
                        report["status"] = "FAIL"
                        report["cleanup_error"] = removed.stderr[-500:]
    except Exception as exc:
        report["status"] = "FAIL"
        report["error"] = str(exc)
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "PASS" and report.get("profile_unloaded") else 1


if __name__ == "__main__":
    raise SystemExit(main())
