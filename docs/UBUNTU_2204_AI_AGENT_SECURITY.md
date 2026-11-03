# Ubuntu 22.04 LTS: security review for AI coding agents

**Status:** defensive research and documentation proposal, not a vulnerability disclosure or a verified Ubuntu patch  
**Release scope:** Ubuntu 22.04 LTS (Jammy Jellyfish)  
**Prepared by:** Charalampos Nadiotis, PELAOS Tech (GitHub: [n0711](https://github.com/n0711))  
**Date:** 2026-11-03

## Why this matters

A coding agent that reads untrusted repository content and can execute shell commands acts with the privileges of the process running it. Instructions embedded in README files, fetched pages, issue comments, dependencies or build output can therefore become a prompt-injection entry point. Installation through a package manager is **not** proof that the agent is sandboxed. Distinguish the agent's model/tool permission prompts from operating-system enforcement.

This document proposes a *defensive* documentation addition to Canonical's Ubuntu Security documentation. It does not claim a CVE, patch the Ubuntu kernel or demonstrate an exploit.

## Ubuntu 22.04-specific facts to preserve

1. Ubuntu's published security feature comparison identifies AppArmor **3.0.4** for 22.04 and **4.0.1** for 24.04. Do not copy a 24.04 AppArmor profile into Jammy and assume the same parser or kernel support.
2. The same comparison does **not** list AppArmor's unprivileged user-namespace restrictions for 22.04. Do not describe those newer defaults as if they were provided by stock Jammy.
3. Canonical's Snap documentation distinguishes **strict** confinement from **classic** (no Snap confinement) and **devmode** (enforcement disabled for debugging). Classic Snap packaging, including an AI developer tool installed with `--classic`, does not provide strict confinement.
4. Strict Snap confinement combines AppArmor, seccomp, device control and namespaces, subject to the effective policies, interfaces, host kernel and snap packaging. A label alone is not an end-to-end security proof.
5. Installed Ubuntu packages and kernel flavors can vary. Check a concrete advisory against **Jammy, the specific source package and installed version** before declaring a machine affected or fixed.

## Threat model

| Surface | Example failure mode | Required defensive boundary |
| --- | --- | --- |
| Repository content | Prompt injection in comments, instructions or tests | Treat repository text as untrusted data, not authorization |
| Shell tool | Agent executes a command beyond the requested task | Require command approval and OS-enforced least privilege |
| Credentials | Agent reads `~/.ssh`, cloud tokens or private `.env` files | Do not expose credentials or secret directories to the agent runtime |
| Working directory | Agent edits unrelated repositories or user files | Narrow writable filesystem mounts to the approved workspace |
| Network | Agent or a dependency transmits data unexpectedly | Use restricted egress when supported and review any required endpoints |
| Dependencies/plugins | Unreviewed extension executes with user privileges | Review provenance, pin versions and audit updates |
| CI | Untrusted pull-request workflow acquires write token | Restrict workflow permissions and separate trusted publishing jobs |
| Sandbox assumptions | Tool has classic Snap access or host Docker socket | Verify actual isolation; do not equate container membership with least privilege |

## Read-only evidence collection on a Jammy test host

The following commands are *inspection only*; avoid recording or publishing secret values:

```bash
. /etc/os-release && printf '%s %s\n' "$ID" "$VERSION_ID"
uname -r
systemd-detect-virt 2>/dev/null || true
command -v aa-status >/dev/null && sudo aa-status || true
command -v snap >/dev/null && snap list || true
command -v snap >/dev/null && snap connections || true
apt-cache policy apparmor snapd
```

For a specific AI-agent snap, inspect its published confinement separately using `snap info SNAP_NAME`. For a specific executable, determine how it was installed and what filesystem, network, shell and token access it receives. Do not report a machine as confined merely because `aa-status` lists *some* loaded profiles.

**Note:** WSL and containerized Ubuntu installations may not share the native-host kernel or AppArmor behavior. Record the execution environment before extrapolating Jammy results.

## Minimum safe configuration for AI coding agents

- Use a **separate unprivileged identity** or a properly configured sandbox for untrusted agent execution; do not launch the agent with `sudo`.
- Allow writes only to a dedicated, version-controlled working directory; keep personal directories, SSH agents, secrets and host administration sockets out of the runtime.
- Disable unnecessary tools, plugins and network connections. Treat the agent's own permission prompts as an additional control, **not** the sole containment boundary.
- Approve system installation, Git pushes, merges and destructive changes in a separate human review step.
- Keep the host kernel, userspace and agent runtime on supported security updates, and monitor Canonical's release-specific advisories.
- Run source and dependency checks in CI before merging. Reproduce security claims with a documented Ubuntu 22.04 test environment.
- Treat logging cautiously: command outputs may contain credentials. Redact before sharing incident reports.

## Proposed validation matrix

| Check | Jammy baseline | Status |
| --- | --- | --- |
| Documentation references match Canonical's release comparison | Canonical documentation | Source-checked |
| Strict, classic and devmode Snap semantics are accurate | Canonical Snap confinement documentation | Source-checked |
| Read-only commands run on a clean Ubuntu 22.04 VM | Jammy VM | **Not executed** |
| Agent cannot read a canary file outside allowed workspace | Jammy VM with configured confinement | **Not executed** |
| Agent cannot write outside approved workspace | Jammy VM with configured confinement | **Not executed** |
| Agent cannot reach prohibited network endpoint when egress is restricted | Jammy VM with configured policy | **Not executed** |
| AppArmor profile loads on Jammy with relevant kernel | Jammy VM | **Not executed** |
| CI fails on unexpected privilege increase | Dedicated test harness | **Not implemented** |

A successful documentation review or synthetic test is not evidence of a sandbox escape being prevented on a real Ubuntu host.

## Suggested upstream submission

The upstream repository [canonical/ubuntu-security-documentation](https://github.com/canonical/ubuntu-security-documentation) already contains `common-mistakes/` and `security-features/privilege-restriction/`. Submit a concise reStructuredText page focused on **AI coding agents and false confinement assumptions**, with a Jammy-specific warning, then seek maintainer feedback before adding prescriptive AppArmor profiles.

A ready-to-review candidate is maintained in this repository at:
`integrations/ubuntu-security-documentation/common-mistakes/ai-coding-agents.rst`.

## References

- [Ubuntu security feature overview by release](https://documentation.ubuntu.com/security/security-features/security-features-overview/)
- [Ubuntu AppArmor](https://documentation.ubuntu.com/security/security-features/privilege-restriction/apparmor/)
- [Ubuntu Snap confinement](https://documentation.ubuntu.com/security/security-features/privilege-restriction/snap-confinement/)
- [Ubuntu: Disabling AppArmor is a common mistake](https://documentation.ubuntu.com/security/common-mistakes/disabling-apparmor/)
- [Ubuntu CVE tracker](https://ubuntu.com/security/cves)
- [Canonical Ubuntu security documentation contribution guide](https://github.com/canonical/ubuntu-security-documentation/blob/main/README.md)

**Attribution:** Copyright 2026 Charalampos Nadiotis, PELAOS Tech. Proposed upstream documentation must follow the upstream project's licensing and contribution process.
