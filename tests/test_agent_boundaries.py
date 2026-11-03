#!/usr/bin/env python3
"""Bounded, non-exploit AI-agent security boundary smoke tests.

All fixtures are synthetic and placed under a disposable temporary directory.
No real secrets, external endpoints, or system installations are touched.
These tests do NOT demonstrate Ubuntu 22.04, Snap, or AppArmor confinement.

Copyright 2026 Charalampos Nadiotis, PELAOS Tech
SPDX-License-Identifier: MIT
"""
import json
import os
from pathlib import Path
import pwd
import shutil
import socket
import subprocess
import sys
import tempfile
import unittest


def command(*args, timeout=5):
    return subprocess.run(args, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                          text=True, timeout=timeout, check=False)


class AgentBoundarySmokeTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.tmp = tempfile.TemporaryDirectory(prefix='pelaos-boundary-')
        cls.root = Path(cls.tmp.name)
        cls.root.chmod(0o755)
        cls.workspace = cls.root / 'workspace'
        cls.workspace.mkdir(mode=0o700)
        cls.protected = cls.root / 'protected'
        cls.protected.mkdir(mode=0o700)
        cls.secret = cls.protected / 'fake-secret.txt'
        cls.secret.write_text('synthetic-canary-only', encoding='utf-8')
        cls.secret.chmod(0o600)
        cls.same_user_secret = cls.root / 'same-user-fake-secret.txt'
        cls.same_user_secret.write_text('synthetic-same-uid-canary', encoding='utf-8')
        cls.same_user_secret.chmod(0o600)
        cls.run_as = None
        if os.geteuid() == 0 and shutil.which('runuser'):
            u = pwd.getpwnam('nobody')
            cls.run_as = ['runuser', '-u', u.pw_name, '--']
            os.chown(cls.workspace, u.pw_uid, u.pw_gid)
            os.chown(cls.same_user_secret, u.pw_uid, u.pw_gid)

    @classmethod
    def tearDownClass(cls):
        cls.tmp.cleanup()

    def require_separate_identity(self):
        if self.run_as is None:
            self.skipTest('Requires disposable test identity and privilege to change UID')

    def test_01_workspace_write_allowed_for_test_identity(self):
        self.require_separate_identity()
        target = self.workspace / 'expected-write.txt'
        p = command(*self.run_as, sys.executable, '-c',
                    'from pathlib import Path; import sys; Path(sys.argv[1]).write_text("workspace-canary")', str(target))
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertEqual(target.read_text(), 'workspace-canary')

    def test_02_protected_secret_read_denied_by_unix_dac(self):
        self.require_separate_identity()
        p = command(*self.run_as, sys.executable, '-c',
                    'from pathlib import Path; import sys; print(Path(sys.argv[1]).read_text())', str(self.secret))
        self.assertNotEqual(p.returncode, 0, 'Protected canary unexpectedly readable')
        self.assertIn('PermissionError', p.stderr)

    def test_03_outside_workspace_write_denied_by_unix_dac(self):
        self.require_separate_identity()
        target = self.protected / 'should-not-exist.txt'
        p = command(*self.run_as, sys.executable, '-c',
                    'from pathlib import Path; import sys; Path(sys.argv[1]).write_text("CANARY")', str(target))
        self.assertNotEqual(p.returncode, 0, 'Protected canary unexpectedly writable')
        self.assertFalse(target.exists())

    def test_04_same_uid_can_read_own_secret_without_sandbox(self):
        """PASS demonstrates the exposure - same UID is not agent isolation."""
        self.require_separate_identity()
        p = command(*self.run_as, sys.executable, '-c',
                    'from pathlib import Path; import sys; print(Path(sys.argv[1]).read_text())',
                    str(self.same_user_secret))
        self.assertEqual(p.returncode, 0, p.stderr)
        self.assertEqual(p.stdout.strip(), 'synthetic-same-uid-canary')

    def test_05_symlink_escape_detected_by_path_resolution(self):
        self.require_separate_identity()
        link = self.workspace / 'out-of-workspace'
        link.symlink_to(self.protected, target_is_directory=True)
        self.assertFalse(link.resolve().is_relative_to(self.workspace.resolve()))
        self.assertTrue((self.workspace / 'safe.txt').resolve().is_relative_to(self.workspace.resolve()))
        # A resolved-path check is not race-safe confinement.

    def test_06_user_namespace_feature_probe(self):
        if not shutil.which('unshare'):
            self.skipTest('unshare utility unavailable')
        p = command('unshare', '--user', '--map-root-user', 'id', '-u')
        if p.returncode:
            self.skipTest('User namespace unavailable: ' + p.stderr.strip()[:160])
        self.assertEqual(p.stdout.strip(), '0')

    def test_07_network_namespace_localhost_separation(self):
        """Tests localhost namespace separation, NOT internet egress filtering."""
        if not shutil.which('unshare'):
            self.skipTest('unshare utility unavailable')
        probe = command('unshare', '--user', '--map-root-user', '--net', 'true')
        if probe.returncode:
            self.skipTest('Network namespace unavailable: ' + probe.stderr.strip()[:160])
        sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        try:
            sock.bind(('127.0.0.1', 0))
            sock.listen(1)
            port = sock.getsockname()[1]
            outsider = socket.create_connection(('127.0.0.1', port), timeout=1)
            outsider.close()
            code = ('import socket,sys; s=socket.socket(); s.settimeout(1); '
                    'sys.exit(1 if s.connect_ex(("127.0.0.1",int(sys.argv[1]))) else 0)')
            p = command('unshare', '--user', '--map-root-user', '--net',
                        sys.executable, '-c', code, str(port))
            self.assertEqual(p.returncode, 1, 'Host namespace loopback port reached inside isolated net namespace')
        finally:
            sock.close()

    def test_08_ubuntu_2204_runtime_and_apparmor_status(self):
        values = {}
        for line in Path('/etc/os-release').read_text().splitlines():
            if '=' in line:
                k, v = line.split('=', 1)
                values[k] = v.strip('"')
        if not (values.get('ID') == 'ubuntu' and values.get('VERSION_ID') == '22.04'):
            self.skipTest('Ubuntu 22.04 unavailable: ' + values.get('PRETTY_NAME', 'unknown'))
        if not Path('/sys/kernel/security/apparmor').exists():
            self.skipTest('AppArmor securityfs unavailable; cannot validate confinement')
        self.assertTrue(Path('/sys/kernel/security/apparmor').exists())


if __name__ == '__main__':
    suite = unittest.defaultTestLoader.loadTestsFromTestCase(AgentBoundarySmokeTests)
    results = []

    class Reporter(unittest.TextTestResult):
        def startTest(self, test):
            super().startTest(test)

        def addSuccess(self, test):
            super().addSuccess(test)
            results.append({'test': test._testMethodName, 'status': 'PASS'})

        def addSkip(self, test, reason):
            super().addSkip(test, reason)
            results.append({'test': test._testMethodName, 'status': 'SKIP', 'reason': reason})

        def addFailure(self, test, err):
            super().addFailure(test, err)
            results.append({'test': test._testMethodName, 'status': 'FAIL'})

        def addError(self, test, err):
            super().addError(test, err)
            results.append({'test': test._testMethodName, 'status': 'ERROR'})

    outcome = unittest.TextTestRunner(verbosity=2, resultclass=Reporter).run(suite)
    if len(sys.argv) > 1:
        payload = {
            'scope': 'bounded smoke tests, not Ubuntu 22.04/AppArmor validation',
            'kernel': os.uname().release,
            'os_release': Path('/etc/os-release').read_text(),
            'tests': results,
            'passed': len([t for t in results if t['status'] == 'PASS']),
            'skipped': len([t for t in results if t['status'] == 'SKIP']),
            'failed': len([t for t in results if t['status'] in ('FAIL', 'ERROR')]),
        }
        Path(sys.argv[1]).write_text(json.dumps(payload, indent=2) + '\n', encoding='utf-8')
    raise SystemExit(0 if outcome.wasSuccessful() else 1)
