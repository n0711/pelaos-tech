#!/usr/bin/env python3
"""Bounded Jammy AppArmor regression tests for benign, same-UID processes.

Disposable GitHub-hosted runner only. All canaries are synthetic. This is not
a deployable AI-agent profile and does not test model behavior or real credentials.
SPDX-License-Identifier: MIT
"""
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
import json
import os
from pathlib import Path
import subprocess
import sys

base = Path(sys.argv[1])

def observe(action):
    try:
        action()
        return "ALLOWED"
    except PermissionError:
        return "DENIED"
    except OSError as exc:
        return "OTHER_ERRNO_" + str(exc.errno)

def shell():
    try:
        p = subprocess.run(["/bin/sh", "-c", ":"], capture_output=True,
                           text=True, timeout=4, check=False)
        return "ALLOWED" if p.returncode == 0 else "NONZERO_" + str(p.returncode)
    except PermissionError:
        return "DENIED"

def python_label():
    p = subprocess.run([sys.executable, "-c",
                        "from pathlib import Path; print(Path('/proc/self/attr/current').read_text().strip())"],
                       capture_output=True, text=True, timeout=4, check=False)
    if p.returncode:
        return "ERROR: " + p.stderr[-160:]
    return p.stdout.strip()

result = {
    "uid": os.geteuid(),
    "label": Path("/proc/self/attr/current").read_text().strip(),
    "canary_direct_read": observe(lambda: (base/"protected"/"canary.txt").read_text()),
    "canary_symlink_read": observe(lambda: (base/"work"/"secret-link").read_text()),
    "canary_dotdot_read": observe(lambda: (base/"work"/".."/"protected"/"canary.txt").read_text()),
    "outside_direct_write": observe(lambda: (base/"outside"/"target.txt").write_text("synthetic")),
    "outside_symlink_write": observe(lambda: (base/"work"/"outside-link").write_text("synthetic")),
    "workspace_write": observe(lambda: (base/"work"/"approved.txt").write_text("synthetic")),
    "shell_exec": shell(),
    "python_child_label": python_label(),
}
print(json.dumps(result))
"""


def run(argv, timeout=15):
    return subprocess.run(argv, capture_output=True, text=True, timeout=timeout, check=False)


def probe(prefix, directory):
    proc = run([*prefix, sys.executable, "-c", CHILD, str(directory)])
    if proc.returncode:
        raise RuntimeError("Probe failed: " + proc.stderr[-600:])
    return json.loads(proc.stdout)


def main():
    report = {
        "suite": "AppArmor benign-agent boundary regressions",
        "os_target": "Ubuntu 22.04",
        "kernel": platform.release(),
        "scope": "Synthetic same-UID file traversal, symlink redirection, exec inheritance",
        "not_tested": ["real AI agent", "external network", "Snap", "host secrets", "native GA kernel"],
        "status": "FAIL",
    }
    try:
        if os.geteuid() != 0:
            raise RuntimeError("Must run only as root in a disposable CI VM")
        release = {
            key: value.strip('"') for key, value in
            (line.split("=", 1) for line in Path("/etc/os-release").read_text().splitlines() if "=" in line)
        }
        if release.get("ID") != "ubuntu" or release.get("VERSION_ID") != "22.04":
            raise RuntimeError("Wrong release for the Jammy CI experiment")
        missing = [name for name in ("apparmor_parser", "aa-exec", "setpriv")
                   if not shutil.which(name)]
        if missing:
            raise RuntimeError("Missing required tools: " + ", ".join(missing))
        user = pwd.getpwnam("nobody")
        with tempfile.TemporaryDirectory(prefix="pelaos-jammy-regression-") as tmp:
            base = Path(tmp)
            os.chown(base, user.pw_uid, user.pw_gid)
            for name in ("work", "outside", "protected"):
                folder = base / name
                folder.mkdir(mode=0o700)
                os.chown(folder, user.pw_uid, user.pw_gid)
            secret = base / "protected" / "canary.txt"
            secret.write_text("NOT-A-SECRET")
            target = base / "outside" / "target.txt"
            target.write_text("NOT-A-SECRET")
            for file in (secret, target):
                os.chown(file, user.pw_uid, user.pw_gid)
                file.chmod(0o600)
            (base / "work" / "secret-link").symlink_to(secret)
            (base / "work" / "outside-link").symlink_to(target)
            profile_name = "pelaos-regression-" + str(os.getpid())
            profile = f"""#include <tunables/global>
profile {profile_name} flags=(attach_disconnected) {{
  #include <abstractions/base>
  /usr/bin/python3* ix,
  /usr/lib/** mr,
  /lib/** mr,
  /etc/** r,
  /proc/** r,
  /dev/** rw,
  {base}/ r,
  {base}/** rw,
  deny {secret} r,
  deny {base}/outside/** w,
}}
"""
            profile_path = base / "fixture.apparmor"
            profile_path.write_text(profile)
            uid_prefix = [
                "setpriv", "--reuid=" + str(user.pw_uid),
                "--regid=" + str(user.pw_gid), "--clear-groups",
            ]
            baseline = probe(uid_prefix, base)
            report["baseline"] = baseline
            baseline_expected = {
                "canary_direct_read": "ALLOWED",
                "canary_symlink_read": "ALLOWED",
                "canary_dotdot_read": "ALLOWED",
                "outside_direct_write": "ALLOWED",
                "outside_symlink_write": "ALLOWED",
                "workspace_write": "ALLOWED",
                "shell_exec": "ALLOWED",
                "uid": user.pw_uid,
                "label": "unconfined",
                "python_child_label": "unconfined",
            }
            if any(baseline.get(k) != v for k, v in baseline_expected.items()):
                raise RuntimeError("Unconfined positive controls invalid, see baseline")
            loaded = False
            try:
                add = run(["apparmor_parser", "-a", str(profile_path)])
                if add.returncode:
                    raise RuntimeError("Cannot load fixture: " + add.stderr[-600:])
                loaded = True
                confined = probe([*uid_prefix, "aa-exec", "-p", profile_name, "--"], base)
                report["confined"] = confined
                expected = {
                    "canary_direct_read": "DENIED",
                    "canary_symlink_read": "DENIED",
                    "canary_dotdot_read": "DENIED",
                    "outside_direct_write": "DENIED",
                    "outside_symlink_write": "DENIED",
                    "workspace_write": "ALLOWED",
                    "shell_exec": "DENIED",
                    "uid": user.pw_uid,
                }
                expected_label = profile_name + " (enforce)"
                checks = {key: confined.get(key) == val for key, val in expected.items()}
                checks["enforcing_label"] = confined.get("label") == expected_label
                checks["python_child_inherits_enforcing_label"] = (
                    confined.get("python_child_label") == expected_label
                )
                report["checks"] = checks
                if not all(checks.values()):
                    raise RuntimeError("One or more regression checks failed")
                report["status"] = "PASS"
            finally:
                if loaded:
                    removed = run(["apparmor_parser", "-R", str(profile_path)])
                    report["fixture_unloaded"] = removed.returncode == 0
                    if removed.returncode:
                        raise RuntimeError("Temporary profile removal failed: " + removed.stderr[-400:])
    except Exception as exc:
        report["status"] = "FAIL"
        report["error"] = str(exc)
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "PASS" and report.get("fixture_unloaded") else 1


if __name__ == "__main__":
    raise SystemExit(main())
