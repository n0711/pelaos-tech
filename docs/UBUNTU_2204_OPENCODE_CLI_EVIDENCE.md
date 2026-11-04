# Ubuntu 22.04 OpenCode CLI - bounded AppArmor and seccomp evidence

**Recorded:** 2026-11-04  
**Author:** Charalampos Nadiotis, PELAOS Tech  
**Repository:** `n0711/pelaos-tech`  
**Successful run:** https://github.com/n0711/pelaos-tech/actions/runs/41828194978  
**Failed exploratory run:** https://github.com/n0711/pelaos-tech/actions/runs/41827956440  
**CI environment:** GitHub-hosted `ubuntu-22.04`, Linux `6.8.0-1070-azure`  
**Native Ubuntu 22.04 GA validation:** Not done

## Artifact provenance

- **Application:** OpenCode CLI, v1.19.5, Linux x64 baseline binary
- **Upstream:** `anomalyco/opencode` GitHub release `v1.19.5`
- **Archive:** `opencode-linux-x64-baseline.tar.gz`
- **SHA256:** `17c200fd4b2289954cc01552559f9b6d03ec157b2ddfb5eb54144f491656333d`
- The GitHub Actions job verified the SHA256 before extracting a single `opencode` executable. The checksum is based on upstream release metadata, not on an independently audited third-party signature.

## Test design

- The test used one disposable user identity, UID `65534`, and a synthetic temporary home and workspace.
- OpenCode received an explicitly restricted environment without provider tokens, repository credentials, real SSH keys or user data.
- Both baseline and confined checks launched the *real OpenCode executable* for `--version` only. Neither invoked an LLM or exercised the agent's shell tools.
- The temporary AppArmor fixture was loaded with `apparmor_parser` and the test subprocess invoked with `aa-exec` under an enforcing profile.
- In the revised confined case, seccomp blocked `socket` and `connect` system calls. The filter was applied before the OpenCode subprocess, which therefore inherited the syscall restriction.
- Cleanup of the AppArmor profile was a hard passing condition.

## Results, same Unix identity

| Tested operation | Unconfined baseline | AppArmor + seccomp |
| --- | --- | --- |
| OpenCode CLI `--version` | Success, `1.19.5` | Success, `1.19.5` |
| Read an owned synthetic protected file | ALLOWED | DENIED |
| Write an owned file outside workspace | ALLOWED | DENIED |
| Write inside approved workspace | ALLOWED | ALLOWED |
| Create an IPv4 stream socket | ALLOWED | DENIED |
| Effective Python fixture AppArmor label | `unconfined` | `pelaos-opencode-ci-1972 (enforce)` |
| Temporary AppArmor profile unloaded | Not applicable | YES |

The final CI test reported `PASS`; all seven expected observations matched.

## Negative evidence and corrective action

The initial AppArmor-only run **FAILED** its network restriction assertion:
although the enforcing AppArmor profile denied the file operations,
an IPv4 socket was still allowed despite a `deny network` rule.

That observation is retained as a test finding. It is not evidence that
AppArmor networking rules are universally ineffective. Kernel configuration,
LSM coverage and profile semantics need separate investigation.

The revised check does not pretend that the AppArmor network rule worked.
Instead, it independently loads a seccomp filter with explicit `socket`
and `connect` restrictions, and tests that the local IPv4 socket creation
fails. No external network traffic is initiated.

## Boundaries of the claim

**Verified:** checksum check for one pinned release archive, OpenCode
`--version` execution with synthetic environment under the inspected
AppArmor profile, file canary checks and a separately tested seccomp
socket restriction.

**Not verified:** interactive model sessions, prompt injection, LLM-generated
tool calls, MCP or other plugin execution, OpenCode's network transport,
DNS, actual provider API connections, outbound traffic control for every
syscall, Snap confinement, native Ubuntu 22.04 installations, user home
protection under real agent workload, or production-ready AppArmor policy.

**Important:** The demonstration test profile is not suitable for deploying
a real AI agent. Do not copy it to `/etc/apparmor.d` on an operational system
without redesign, review and validation.

## Follow-up acceptance criteria

1. Run a real OpenCode tool workflow with an offline/local test model on a
   disposable VM, inspecting shell subprocess confinement and label inheritance.
2. Establish a narrower, reviewed runtime AppArmor policy with explicit
   handling for the executable, modules, caches, child processes and required
   network behavior.
3. Independently test Snap strict/classic package metadata and real execution
   on Ubuntu 22.04, with release and revision provenance.
4. Repeat relevant checks on a native Ubuntu 22.04 VM with recorded kernel
   flavor and AppArmor audit evidence.
