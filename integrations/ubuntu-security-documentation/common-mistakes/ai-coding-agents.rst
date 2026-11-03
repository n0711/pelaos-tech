AI coding agents and confinement assumptions
############################################

AI coding agents can read repository files, call command-line tools and perform
actions in the developer's working environment. Content retrieved from a repository
or a website is untrusted input. An agent's confirmation prompt or model
instructions are not an operating-system sandbox.

A process that executes with the developer's identity may be able to access files
and tokens that the developer can access. Restricting a working directory in an
agent's settings does not by itself enforce filesystem isolation.

What to check
=============

* Identify the agent executable, its origin, version and execution identity.
* Keep credentials, SSH material and other private files outside the agent's
  accessible environment. A separate low-privilege account or independently
  enforced sandbox can help reduce exposure.
* Limit writable directories, privileged tools and network destinations to those
  necessary for the approved task.
* Review extensions, dependencies and commands obtained from untrusted content.
* Require an independent approval step for privileged operations, repository
  publication and destructive changes.
* Verify effective restrictions with benign tests that attempt to read and write
  synthetic canary files. Do not use real secrets for these tests.

Snap confinement is not universal
=================================

Snap packages may have strict, classic or devmode confinement. Strict confinement
uses several kernel security mechanisms, including AppArmor and seccomp, subject
to available host features and connected interfaces. Classic snaps do not receive
strict Snap confinement. Devmode is intended for debugging and does not enforce
the normal strict restrictions.

Installing an AI tool as a classic snap is therefore not evidence that the tool
is isolated from the user's files or processes.

Ubuntu 22.04 LTS considerations
===============================

Ubuntu 22.04 LTS and Ubuntu 24.04 LTS do not have identical security defaults.
The Ubuntu security features overview lists AppArmor 3.0.4 for 22.04 and
AppArmor 4.0.1 for 24.04, and does not list the newer unprivileged
user-namespace restrictions as a 22.04 default.

Do not assume that an AppArmor profile written for a newer release loads or
enforces identically on 22.04. Capture the running kernel, installed packages,
actual profile attachment and effective denial logs before reporting a control
as implemented.

Limitations of a local test
===========================

A successful Unix file-permission denial establishes only that particular
identity and path were denied. It does not prove that all paths, symlinks,
network destinations or tools are contained. User-namespace and network-namespace
availability alone do not constitute a full sandbox. Repeat tests under the
specific documented enforcement policy on a representative 22.04 host.

The PELAOS Tech smoke-test record was executed on Debian 13, not Jammy.
It is not evidence of AppArmor confinement on Ubuntu 22.04.

Sources and related guidance
============================

* https://documentation.ubuntu.com/security/security-features/security-features-overview/
* https://documentation.ubuntu.com/security/security-features/privilege-restriction/snap-confinement/
* https://github.com/n0711/pelaos-tech/blob/docs/ubuntu-2204-ai-agent-security-20261103/docs/UBUNTU_2204_AI_AGENT_SMOKE_RESULTS_2026-11-03.md
* :doc:`../security-features/privilege-restriction/apparmor`
* :doc:`disabling-apparmor`
* :doc:`deb-download-untrusted`
