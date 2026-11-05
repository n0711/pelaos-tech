# Ubuntu 22.04 - Claude Code and Codex CLI bounded security validation

**Date:** 2026-11-05  
**Project:** PELAOS Tech, `n0711/pelaos-tech`  
**Author:** Charalampos Nadiotis  
**Verified GitHub Actions run:** https://github.com/n0711/pelaos-tech/actions/runs/41967435972  
**Verified branch commit:** `995eb10c72d4a4f16e7fcb8f6b547c600285d963`

## Objective and environment

Validate that *named real coding CLIs* can launch in a disposable same-UID
AppArmor fixture while independent, benign file-operation probes obey
expected restrictions. This is not an evaluation of an authenticated
agent session or LLM-generated tool actions.

- Hosted runner: GitHub Actions `ubuntu-22.04`, Ubuntu 22.04.5 LTS
- Kernel: Linux `6.8.0-1070-azure`, **not** the original GA kernel
- Tested identity: UID `65534` (`nobody`) in baseline and confined phases
- Install method: pinned official npm package release under `$RUNNER_TEMP`,
  copied to disposable synthetic fixture for UID separation
- Node.js major: 22, configured by GitHub Actions
- Model authentication: **none**. No provider token or real credentials supplied
- Test entry points: `claude --version` and `codex --version` only
- AppArmor: temporary policy parsed by `apparmor_parser`, attached via
  `aa-exec` and removed after the tests
- Separate seccomp: `socket` and `connect` filtered in the test child before
  CLI launch, `socketpair` permitted because Codex startup uses local signalling
- No calls to real model endpoints; no network destination contacted by the
  explicit test harness

## Package provenance and tested versions

| CLI | Npm spec pinned by workflow | Observed version |
| --- | --- | --- |
| Claude Code | `@anthropic-ai/claude-code@2.1.311` | `2.1.311 (Claude Code)` |
| Codex CLI | `@openai/codex@0.166.0` | `codex-cli 0.166.0` |

The npm client validates package metadata/integrity during installation, but
the workflow does **not** supply independent artifact SHA256 or a reviewed
software supply-chain attestation. This is an explicit provenance limitation.

## Same-UID acceptance results

| Operation | Unconfined baseline | AppArmor fixture (both CLIs) |
| --- | --- | --- |
| Run named CLI `--version` | PASS | PASS |
| Read owned synthetic protected canary | ALLOWED | DENIED |
| Read same canary via workspace symlink | ALLOWED | DENIED |
| Write owned file outside approved workspace | ALLOWED | DENIED |
| Write same outside file via symlink | ALLOWED | DENIED |
| Write inside synthetic approved workspace | ALLOWED | ALLOWED |
| Execute `/bin/sh -c ':'` from fixture | ALLOWED | DENIED |
| Effective fixture label | `unconfined` | `pelaos-cli-<name>-<pid> (enforce)` |
| Create IPv4 socket **before** seccomp filter | ALLOWED | ALLOWED |
| Create IPv4 socket **after** seccomp filter | DENIED | DENIED |
| Temporary AppArmor policy removed | Not applicable | YES |

All twelve confined assertions passed for both named CLIs. The
separately archived JSON reports contain the exact effective label names
and individual boolean assertions. GitHub reports both matrix jobs and
the workflow as **SUCCESS**.

### Failed exploratory attempts retained

Earlier revisions did **not** pass. The progression is retained in the PR
history and corresponding GitHub Actions runs:

1. An invalid `rmi` AppArmor permission was rejected by `apparmor_parser`.
2. Codex refused helper creation under `/tmp`; moving to `/home/runner`
   failed same-UID directory traversal because the parent is not
   world-accessible. The disposable fixture moved to `/var/tmp`.
3. After path fixes, the Codex CLI encountered a local signal-initialization
   issue when seccomp also blocked `socketpair`. The final policy blocks
   `socket` and `connect` only. This allows Codex to start while the
   explicit AF_INET socket test still observes denial.
4. One transient CI revision contained a Python syntax error; the final
   revision was repaired and both final matrix jobs completed successfully.

None of these observations is evidence of an exploitable vulnerability in
Claude Code, Codex CLI or Ubuntu. They are limitations and corrections to
the disposable test design.

## Scope and interpretation

**Verified:** the selected pinned CLI binaries ran `--version` successfully
under the low-privilege same-UID test arrangement. The Python fixture's
effective AppArmor label was `(enforce)`, with specific file operation
denials and an allowed workspace write. The CLI launch occurred within
that labelled process through an `ix` execution rule; the CLI's own label
was not sampled from `/proc/<cli-pid>/attr/current` during its brief
execution. A separate seccomp syscall filter denied the local IPv4
socket probe after installation.

**Not verified:** authenticated Claude Code or Codex sessions, model
prompt injection, agent-driven shell/tool/MCP subprocess containment,
real credential exposure, all possible file paths, real network egress
or DNS policy, standalone AppArmor network filtering, native Jammy GA/HWE
host diversity, Snap strict/classic behavior, downstream dependency
signatures, or production least-privilege agent profiles.

**Do not deploy this demonstration AppArmor policy to an operational host.**
The policy grants broad interpreter and system-read access for test
bootstrapping and has not been subjected to production policy review.

## Next acceptance criteria

1. Explicitly distinguish the parent Python fixture label from the CLI
   executable's own label. Observe a long-lived, benign child with
   `/proc/<pid>/attr/current` to verify inheritance directly.
2. Use a named offline/local model test to exercise an authentic agent
   tool subprocess under the policy, without real provider credentials.
3. Review narrower policy permissions, audit logs and a native Ubuntu 22.04
   VM's kernel/AppArmor configuration.
4. Test separate, controlled network restrictions, not only the local
   `socket`/`connect` syscall fixture.
5. Test actual strict and classic Snap package execution separately,
   identifying exact package revisions and interfaces.
