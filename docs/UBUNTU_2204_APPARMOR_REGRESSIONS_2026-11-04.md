# Ubuntu 22.04 AppArmor path and execution regression evidence

**Date:** 2026-11-04  
**Author:** Charalampos Nadiotis, PELAOS Tech  
**Repository:** `n0711/pelaos-tech`  
**CI:** https://github.com/n0711/pelaos-tech/actions/runs/41781417131  
**Tested commit:** `12016f68c780a850f68241815405460e945c0041`  
**Runtime:** GitHub-hosted Ubuntu 22.04 runner, Linux `6.8.0-1070-azure`

## Objective

Check that the AppArmor kernel policy enforces defined restrictions for a
benign Python fixture when the test identity can access every synthetic file
without confinement. This extends the earlier direct-file test to symlinks,
`..` traversal, process execution and child-label inheritance.

## Observed comparison

Both processes ran with **UID 65534**. The unconfined baseline verified
all controls were accessible to that same Unix identity first.

| Operation | Unconfined | AppArmor-enforced |
| --- | --- | --- |
| Read own synthetic protected file directly | ALLOWED | DENIED |
| Read same canary through workspace symlink | ALLOWED | DENIED |
| Read canary through `..` traversal | ALLOWED | DENIED |
| Write owned outside-workspace file directly | ALLOWED | DENIED |
| Write owned outside-workspace file through symlink | ALLOWED | DENIED |
| Write approved workspace file | ALLOWED | ALLOWED |
| Run `/bin/sh -c ':'` | ALLOWED | DENIED |
| Launch permitted Python subprocess | `unconfined` | `pelaos-regression-2325 (enforce)` |

Test process label: `pelaos-regression-2325 (enforce)`.
**All ten asserted checks passed**. The transient profile was unloaded
successfully after the tests. The result was `PASS`, with the complete JSON
test report archived as a GitHub Actions workflow artifact.

## Why this is stronger than the earlier result

- Each denial was contrasted against an unconfined success with the **same
  Unix UID**, so the individual result is not merely explained by a DAC
  mismatch.
- The symlink and `..` probes use real file operations rather than a
  user-space path-resolution assertion.
- A benign process attempted an ordinary shell command, which was denied
  by the test profile. A permitted Python child inherited the enforcing label.
- CI checks the exact expected observations and fails if the result or
  profile cleanup is not successful.

## Limitations

The fixture policy includes broad read rules to allow the Python interpreter
to start and is **not suitable as a production AI-agent sandbox**. The
experiment does not establish complete protection from arbitrary Python code,
shell-equivalent behavior, hardlink edge cases, open-file-descriptor passing,
confused deputies, real secrets, network exfiltration, container/device
boundaries, kernel privilege escalation, or AppArmor policy consistency
across kernels.

The test did not install, run or validate a real AI coding agent, strict Snap,
classic Snap, user-owned development host or native Ubuntu GA kernel.

## Next steps

1. Prepare an independently reviewed, narrower profile for a **named agent
   executable** in a disposable VM, with test-only identity and synthetic
   home. Validate both permitted operation and rejected operation.
2. Validate audit evidence for denials (subject to runner log access).
3. Verify network egress restrictions with a local-only fixture and
   explicit allow/deny control cases, before claiming network isolation.
4. Check proposed Canonical reStructuredText in an upstream-compatible Sphinx
   environment, with repository documentation conventions and maintainer review.
