# Linux kernel kfifo janitor patch - 9 November 2026

**Status:** AI-assisted draft patch for human review. Not emailed or merged upstream. A human contributor must review the changes and personally add a `Signed-off-by` tag before submitting.

The patch is authored under Charalampos Nadiotis (GitHub `n0711`) and is stored in `integrations/linux-kernel/0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch`. It is **not** a patch to the PELAOS TECH website. It is meant to apply in a Linux kernel tree.

## Change

- `samples/kfifo/bytestream-example.c`: two `printk()` calls use `%u` rather than `%d` for `unsigned int ret`.
- `samples/kfifo/inttype-example.c`: the same two replacements.
- No FIFO behavior, control flow, or API changes.

Upstream reference: Linux `torvalds/linux` tree at commit `7475ea8b58616b33c04afb976cf0fbaad9dd63fb` (Linux 7.4-rc2, 9 November 2026).

Original Git blob hashes, checked against the fetched upstream files:
- `samples/kfifo/bytestream-example.c`: `4ae29a12cc8a03aa2a5c55992f55eaed35982c70`.
- `samples/kfifo/inttype-example.c`: `e4f93317c5d049a337d39f2280c6fe52b45d3010`.

## Validation and submission gates

The four format-string changes were verified against the fetched upstream source text. This review has not established that a complete current Linux kernel build passes or that `checkpatch.pl` passes. Previously reported local test results should be independently reproduced before submission.

**Required steps before sending:**

1. Obtain a complete Linux kernel checkout matching the target source commit and apply the patch.
2. Run `git apply --check`, `scripts/checkpatch.pl`, and `scripts/get_maintainer.pl`.
3. Build or test the changed modules with the target kernel configuration, and inspect any warnings.
4. Review the AI-assisted patch in full, the copyright/licensing basis and the DCO, then **personally add** the human `Signed-off-by` line. An AI agent must not supply it.
5. Confirm that the sending email address and Git author identity are yours.
6. Run `git send-email --dry-run`, inspect recipient addresses, and **send the patch yourself** with `git send-email` only after review. Do not send as an attachment.

Attribution: `Assisted-by: ChatGPT:GPT-6`, reflecting that ChatGPT suggested the patch and changelog.

Kernel AI guidance: https://docs.kernel.org/process/coding-assistants.html
Kernel submission guide: https://docs.kernel.org/process/submitting-patches.html

## Suggested commands from a local Linux checkout

Copy the patch into the Linux checkout or provide its absolute path, then:

```sh
git apply --check 0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch
./scripts/checkpatch.pl --no-tree --strict 0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch
./scripts/get_maintainer.pl 0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch

git send-email --dry-run \
  --from='Charalampos Nadiotis <nadiotis_c@outlook.com>' \
  --to=stefani@seibold.net \
  --cc=linux-kernel@vger.kernel.org \
  0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch
```

After performing the human review and adding your own DCO sign-off, remove `--dry-run` to submit by SMTP with `git send-email`. AI assistance does not certify the DCO or replace the required human submission. The KFIFO maintainer and Linux mailing-list addresses are a starting point, not a substitute for the current maintainer script. If using `git am`, use a clean branch to ensure the patch is not applied twice.

## Upstream references

- [Linux patch submission guide](https://docs.kernel.org/process/submitting-patches.html)
- [Linux patch submission checklist](https://docs.kernel.org/process/submit-checklist.html)
- [Kernel MAINTAINERS source](https://github.com/torvalds/linux/blob/master/MAINTAINERS)

Do not describe this as a merged or accepted kernel contribution until upstream acceptance is verified.
