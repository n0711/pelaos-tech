#!/usr/bin/env python3
"""Ubuntu 22.04 CI: bounded AppArmor MAC check with synthetic same-UID canaries.

Intended for a disposable GitHub-hosted VM, executed as root to load a transient
test-only profile. No real secrets, external network access or agent runtime.
This is NOT a production AppArmor profile or full AI-agent containment proof.
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


def execute(argv):
    return subprocess.run(argv, text=True, capture_output=True, timeout=15, check=False)


def result_for(argv, base):
    cp = execute([*argv, sys.executable, "-c", CHILD, str(base)])
    if cp.returncode:
        raise RuntimeError("test child failed: " + cp.stderr[-700:])
    return json.loads(cp.stdout)


CHILD = r"""
import json
import os
from pathlib import Path
import sys

base = Path(sys.argv[1])
def observation(op):
    try:
        op()
        return "ALLOWED"
    except PermissionError:
        return "DENIED"
out = {
    "canary_read": observation(lambda: (base / "protected" / "canary.txt").read_text()),
    "workspace_write": observation(lambda: (base / "work" / "ok.txt").write_text("synthetic")),
    "outside_write": observation(lambda: (base / "outside" / "blocked.txt").write_text("synthetic")),
    "uid": os.geteuid(),
    "label": Path("/proc/self/attr/current").read_text().strip(),
}
print(json.dumps(out))
"""


def main():
    report = {
        "scope": "Ubuntu 22.04 disposable-VM AppArmor same-UID canary check",
        "limitations": "No real AI agent, no Snap confinement, no internet egress test, test policy is not production safe",
        "kernel": platform.release(),
        "result": "FAIL",
    }
    try:
        if os.geteuid() != 0:
            raise RuntimeError("Requires root on a disposable CI runner to load the test profile")
        data = dict(
            line.split("=", 1) for line in Path("/etc/os-release").read_text().splitlines()
            if "=" in line
        )
        if data.get("ID", "").strip('"') != "ubuntu" or data.get("VERSION_ID", "").strip('"') != "22.04":
            raise RuntimeError("This test is specifically scoped to Ubuntu 22.04")
        required = ("apparmor_parser", "aa-exec", "setpriv")
        missing = [tool for tool in required if not shutil.which(tool)]
        if missing:
            raise RuntimeError("Missing required commands: " + ", ".join(missing))
        test_user = pwd.getpwnam("nobody")
        with tempfile.TemporaryDirectory(prefix="pelaos-aa-ci-") as directory:
            base = Path(directory)
            os.chown(base, test_user.pw_uid, test_user.pw_gid)
            for section in ("protected", "work", "outside"):
                path = base / section
                path.mkdir(mode=0o700)
                os.chown(path, test_user.pw_uid, test_user.pw_gid)
            canary = base / "protected" / "canary.txt"
            canary.write_text("SYNTHETIC-NOT-A-SECRET\n")
            os.chown(canary, test_user.pw_uid, test_user.pw_gid)
            canary.chmod(0o600)
            profile_name = "pelaos-aa-ci-" + str(os.getpid())
            # This test profile deliberately permits only known runtime reads
            # plus synthetic fixture paths. Not a deployable agent profile.
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
  deny {canary} r,
  deny {base}/outside/** w,
}}
"""
            profile_file = base / "profile.apparmor"
            profile_file.write_text(profile)
            setpriv = [
                "setpriv", "--reuid=" + str(test_user.pw_uid),
                "--regid=" + str(test_user.pw_gid), "--clear-groups",
            ]
            baseline = result_for(setpriv, base)
            report["baseline"] = baseline
            expected_baseline = all(
                baseline.get(key) == "ALLOWED"
                for key in ("canary_read", "workspace_write", "outside_write")
            ) and baseline.get("uid") == test_user.pw_uid
            if not expected_baseline:
                raise RuntimeError("Same-UID positive control failed; DAC comparison invalid")
            loaded = False
            try:
                cp = execute(["apparmor_parser", "-a", str(profile_file)])
                if cp.returncode:
                    raise RuntimeError("AppArmor policy load failed: " + cp.stderr[-700:])
                loaded = True
                confined = result_for(
                    [*setpriv, "aa-exec", "-p", profile_name, "--"], base
                )
                report["confined"] = confined
                expect = {
                    "canary_read": "DENIED",
                    "workspace_write": "ALLOWED",
                    "outside_write": "DENIED",
                    "uid": test_user.pw_uid,
                }
                valid = all(confined.get(key) == value for key, value in expect.items())
                valid = valid and confined.get("label", "").startswith(profile_name + " (enforce)")
                if not valid:
                    raise RuntimeError("Effective enforcing label or expected allow/deny checks failed")
                report["result"] = "PASS"
            finally:
                if loaded:
                    cleanup = execute(["apparmor_parser", "-R", str(profile_file)])
                    report["profile_unloaded"] = cleanup.returncode == 0
                    if cleanup.returncode:
                        report["result"] = "FAIL"
                        report["cleanup_error"] = cleanup.stderr[-500:]
    except Exception as exc:
        report["result"] = "FAIL"
        report["error"] = str(exc)
    print(json.dumps(report, indent=2))
    return 0 if report["result"] == "PASS" and report.get("profile_unloaded") else 1


if __name__ == "__main__":
    raise SystemExit(main())
