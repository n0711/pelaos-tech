# Ubuntu 22.04 AppArmor enforcement - verified CI evidence

**Date:** 2026-11-04  
**Project:** PELAOS Tech, n0711/pelaos-tech  
**Author:** Charalampos Nadiotis  
**Evidence:** https://github.com/n0711/pelaos-tech/actions/runs/41779982817  
**Commit tested:** `73b8c74ee9b7b3f87e5ad2c74583a2b47f12b8ac`

## Executed environment

- GitHub-hosted `ubuntu-22.04` runner, reported `Ubuntu 22.04.5 LTS`
- Linux kernel `6.8.0-1070-azure`, **not** Ubuntu 22.04's original GA kernel
- AppArmor was loaded and 41 system profiles were reported in enforce mode before the new check
- `apparmor_parser`, `aa-exec` and `setpriv` were used for the actual attachment test
- No real credentials, agent executable, Snap package or external network endpoint was accessed

## Same-UID MAC acceptance test

The same Unix user, UID 65534 (`nobody`), owned the synthetic files in both
the baseline and the confined process. The disposable test policy was loaded
by `apparmor_parser`, the test process was launched with `aa-exec -p` and
its effective kernel label was read from `/proc/self/attr/current`.

| Observable operation | Baseline (unconfined) | Test AppArmor profile |
| --- | --- | --- |
| Read synthetic owned canary | ALLOWED | DENIED |
| Write synthetic workspace file | ALLOWED | ALLOWED |
| Write synthetic outside-workspace file | ALLOWED | DENIED |
| Unix UID | 65534 | 65534 |
| Effective AppArmor label | `unconfined` | `pelaos-aa-ci-2082 (enforce)` |
| Test profile unloaded after run | Not applicable | YES |

**CI conclusion:** PASS. Unlike the earlier DAC-only test, the baseline confirms
that the relevant operations were available to the same Unix identity before
enforced confinement. This demonstrates targeted AppArmor file-policy denials
on the specified GitHub-hosted Ubuntu runner.

The existing portable test also passed on this run:

- Protected read blocked by Unix DAC for a separate UID
- Approved workspace write allowed
- Outside-workspace write blocked by Unix DAC
- Seccomp `connect` restriction blocked a loopback TCP connection with `EPERM`
- Unrestricted loopback TCP control connected

## Scope and safety

The AppArmor policy exists solely to test known synthetic paths, is loaded on a
disposable CI VM and is removed at the end of the run. **Do not use it as a
production AI-agent security profile.** It has not been evaluated for
race-resistant path isolation, real agent shell/tool behavior, credential
exposure, network egress policies, Snap strict versus classic confinement,
AppArmor audit logs, reboot persistence, or native Jammy GA/HWE host diversity.

## Next acceptance criteria

1. Test a **specific AI agent** under a reviewed and explicitly least-privilege
   AppArmor policy, with synthetic secrets and no real tokens mounted.
2. Verify real network egress policy by a controlled local-only fixture, with
   correct DNS/loopback behavior, and distinguish seccomp from network policy.
3. Test a real strict Snap and classic Snap, record versions, revision and
   actual connected interfaces without assuming the package label is isolation.
4. Repeat the same fixtures on a native Ubuntu 22.04 VM, record kernel/flavor,
   loaded profile and denial audit evidence.
5. Review upstream reStructuredText separately, including Sphinx build checks.
