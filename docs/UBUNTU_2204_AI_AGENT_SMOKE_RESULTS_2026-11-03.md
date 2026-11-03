# AI-agent boundary smoke test record - 2026-11-03

**Author:** Charalampos Nadiotis (PELAOS Tech)  
**Scope:** Defensive, non-exploit tests of Linux filesystem and namespace mechanics in a disposable environment.  
**Executed on:** Debian GNU/Linux 13 (trixie), Linux kernel 6.18.57, Python 3.13.5.  
**Target release:** Ubuntu 22.04 LTS - **not available in the test environment**.

## Results

Command: `python3 tests/test_agent_boundaries.py results.json`

| ID | Observable outcome | Status |
| --- | --- | --- |
| 01 | Separate test identity wrote to its approved workspace | PASS |
| 02 | Separate test identity was denied access to a protected synthetic secret | PASS |
| 03 | Separate test identity was denied an out-of-workspace write by Unix DAC | PASS |
| 04 | Same-UID process could read the synthetic secret owned by that UID | PASS - **exposure demonstrated**, not a security barrier |
| 05 | Path resolution detected a symlink escape outside the approved workspace | PASS - path check only, no race-resistance claim |
| 06 | User namespace creation succeeded | PASS - feature availability only |
| 07 | Ephemeral TCP listener on host loopback was unreachable from a new network namespace | PASS - **not** a general network egress test |
| 08 | Ubuntu 22.04 runtime and effective AppArmor confinement were available | SKIP - environment is Debian 13; AppArmor securityfs unavailable |

**Summary:** 7 PASS, 1 SKIP, 0 FAIL. This is a preliminary smoke-test result, not an Ubuntu kernel, sandbox, Snap, OpenCode or AppArmor security certification.

## Interpretation

A normal same-user AI agent can access material readable by that user unless separately isolated. File permissions protect files belonging to another Unix identity; a different UID is a useful but not sufficient safeguard. The network namespace check tests namespace separation for a local socket, not restricted internet egress, DNS handling, interface setup or an actual agent process.

The scripts create only fabricated test canaries inside a temporary directory. They do not scan real secrets, create remote connections, install packages, run exploit code or modify host security configuration.

## Remaining Ubuntu 22.04 acceptance tests

1. Record an Ubuntu 22.04 image identifier/digest, running kernel and flavor, installed AppArmor and Snap versions, and native VM versus WSL/container context.
2. Apply a documented, Jammy-compatible least-privilege confinement policy for a **benign test program**.
3. Repeat canary reads, outside-workspace writes and network-namespace isolation under that enforced policy. Collect allow/deny records and cleanup evidence.
4. Verify effective AppArmor profile attachment and explicit deny events; do not treat `aa-status` alone as proof.
5. Verify behavior of a chosen **strict** Snap versus a **classic** Snap with concrete revision metadata. Do not assume Snap installation implies isolation.
6. Run documentation reStructuredText/Sphinx lint and upstream checks; request Canonical review before describing any profile as recommended.

## Sources

- Canonical release comparison: https://documentation.ubuntu.com/security/security-features/security-features-overview/
- Canonical Snap confinement: https://documentation.ubuntu.com/security/security-features/privilege-restriction/snap-confinement/
- Canonical 2026-11-02 CVE update: https://ubuntu.com/blog/fixes-available-in-proposed-pocket-for-cve-2026-98432
- Security note: `docs/UBUNTU_2204_AI_AGENT_SECURITY.md`

**Reproducibility:** Run `python3 tests/test_agent_boundaries.py evidence.json`. Skipped tests are expected on environments lacking the required capabilities. Interpret the scope and skips before comparing results.
