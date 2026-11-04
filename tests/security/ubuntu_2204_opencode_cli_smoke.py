#!/usr/bin/env python3
"""OpenCode CLI version under an ephemeral AppArmor policy on Ubuntu 22.04 CI.

This is a bounded release-binary smoke test with fake files and no API keys.
It is NOT validation of actual prompts, tool calls, plugin loading or full
sandbox containment. Never use this demonstration policy for production.
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
import json, os, pathlib, socket, subprocess, sys
root = pathlib.Path(sys.argv[1])
version = sys.argv[2]
exe = root / "opencode"

def observe(operation):
    try:
        operation()
        return "ALLOWED"
    except PermissionError:
        return "DENIED"
    except OSError as exc:
        return f"ERRNO_{exc.errno}"

def create_socket():
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM):
        pass

p = subprocess.run(
    [str(exe), "--version"], text=True, capture_output=True,
    cwd=root / "work", timeout=18, check=False
)
result = {
    "uid": os.geteuid(),
    "label": pathlib.Path("/proc/self/attr/current").read_text().strip(),
    "cli_exit": p.returncode,
    "cli_version": (p.stdout + p.stderr).strip()[:240],
    "cli_version_matches": p.returncode == 0 and version in (p.stdout + p.stderr),
    "canary_read": observe(lambda: (root / "protected" / "synthetic.txt").read_text()),
    "outside_write": observe(lambda: (root / "outside" / "target.txt").write_text("fixture")),
    "workspace_write": observe(lambda: (root / "work" / "approved.txt").write_text("fixture")),
    "ipv4_socket": observe(create_socket),
}
print(json.dumps(result))
"""


def run(command: list[str], *, env=None, timeout=25):
    return subprocess.run(command, text=True, capture_output=True, timeout=timeout, env=env)


def probe(prefix: list[str], root: Path, version: str, uid: int):
    # The subprocess receives no CI token, real home directory or model credentials.
    clean_env = {
        "PATH": "/usr/bin:/bin",
        "HOME": str(root / "work"),
        "XDG_CONFIG_HOME": str(root / "work" / "config"),
        "XDG_CACHE_HOME": str(root / "work" / "cache"),
        "XDG_DATA_HOME": str(root / "work" / "data"),
        "LANG": "C.UTF-8",
        "NO_COLOR": "1",
    }
    p = run([*prefix, sys.executable, "-c", CHILD, str(root), version], env=clean_env)
    if p.returncode:
        raise RuntimeError("Probe crashed: " + p.stderr[-600:])
    data = json.loads(p.stdout)
    if data["uid"] != uid:
        raise RuntimeError("Unexpected test UID")
    return data


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--archive-binary", required=True, type=Path)
    parser.add_argument("--version", required=True)
    args = parser.parse_args()

    report = {
        "suite": "OpenCode pinned release binary CLI AppArmor smoke test",
        "version_requested": args.version,
        "kernel": platform.release(),
        "claim_limit": "CLI --version and synthetic probes only, not an agent/session isolation proof",
        "status": "FAIL",
    }
    try:
        if os.geteuid() != 0:
            raise RuntimeError("Needs root on a disposable CI VM to load/unload an AppArmor fixture")
        facts = {
            k: v.strip('"') for k, v in
            (line.split("=", 1) for line in Path("/etc/os-release").read_text().splitlines()
             if "=" in line)
        }
        if facts.get("ID") != "ubuntu" or facts.get("VERSION_ID") != "22.04":
            raise RuntimeError("Must run on Ubuntu 22.04")
        if not args.archive_binary.is_file():
            raise RuntimeError("The SHA256-verified release executable was not supplied")
        tools = ("setpriv", "aa-exec", "apparmor_parser")
        if any(shutil.which(t) is None for t in tools):
            raise RuntimeError("Missing a required privilege/AppArmor utility")
        user = pwd.getpwnam("nobody")
        with tempfile.TemporaryDirectory(prefix="pelaos-opencode-") as dirname:
            root = Path(dirname)
            root.chmod(0o755)
            for section in ("work", "protected", "outside"):
                directory = root / section
                directory.mkdir(mode=0o700)
                os.chown(directory, user.pw_uid, user.pw_gid)
            canary = root / "protected" / "synthetic.txt"
            target = root / "outside" / "target.txt"
            for file in (canary, target):
                file.write_text("SYNTHETIC-NOT-A-SECRET")
                os.chown(file, user.pw_uid, user.pw_gid)
                file.chmod(0o600)
            binary = root / "opencode"
            shutil.copyfile(args.archive_binary, binary)
            binary.chmod(0o755)
            prefix = [
                "setpriv", "--reuid=" + str(user.pw_uid),
                "--regid=" + str(user.pw_gid), "--clear-groups",
                "--no-new-privs",
            ]
            baseline = probe(prefix, root, args.version, user.pw_uid)
            report["baseline"] = baseline
            required_baseline = {
                "label": "unconfined",
                "canary_read": "ALLOWED",
                "outside_write": "ALLOWED",
                "workspace_write": "ALLOWED",
                "ipv4_socket": "ALLOWED",
                "cli_version_matches": True,
            }
            if any(baseline.get(k) != v for k, v in required_baseline.items()):
                raise RuntimeError("Unconfined positive control failed")
            profile_name = "pelaos-opencode-ci-" + str(os.getpid())
            # Minimal demonstration for startup; still NOT production-ready.
            profile = f"""#include <tunables/global>
profile {profile_name} flags=(attach_disconnected) {{
  #include <abstractions/base>
  /usr/bin/python3* ix,
  /usr/lib/** mr,
  /lib/** mr,
  /lib64/** mr,
  /etc/os-release r,
  /etc/localtime r,
  /etc/ld.so.cache r,
  /etc/passwd r,
  /proc/** r,
  /sys/devices/system/cpu/** r,
  /dev/null rw,
  /dev/urandom r,
  {root}/ r,
  {binary} ix,
  {root}/work/ rw,
  {root}/work/** rwk,
  {root}/protected/ r,
  {canary} r,
  {root}/outside/ r,
  {target} rw,
  deny {canary} r,
  deny {root}/outside/** w,
  deny network,
}}
"""
            policy = root / "test-profile.apparmor"
            policy.write_text(profile)
            loaded = False
            try:
                p = run(["apparmor_parser", "-a", str(policy)])
                if p.returncode:
                    raise RuntimeError("AppArmor fixture failed to load: " + p.stderr[-500:])
                loaded = True
                confined = probe(
                    [*prefix, "aa-exec", "-p", profile_name, "--"],
                    root, args.version, user.pw_uid,
                )
                report["confined"] = confined
                expected = {
                    "label": profile_name + " (enforce)",
                    "canary_read": "DENIED",
                    "outside_write": "DENIED",
                    "workspace_write": "ALLOWED",
                    "ipv4_socket": "DENIED",
                    "cli_version_matches": True,
                }
                report["assertions"] = {
                    k: confined.get(k) == v for k, v in expected.items()
                }
                report["status"] = "PASS" if all(report["assertions"].values()) else "FAIL"
            finally:
                if loaded:
                    cleanup = run(["apparmor_parser", "-R", str(policy)])
                    report["profile_unloaded"] = cleanup.returncode == 0
                    if cleanup.returncode:
                        report["status"] = "FAIL"
                        report["cleanup_error"] = cleanup.stderr[-400:]
    except Exception as exc:
        report["error"] = str(exc)
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "PASS" and report.get("profile_unloaded") else 1


if __name__ == "__main__":
    raise SystemExit(main())
