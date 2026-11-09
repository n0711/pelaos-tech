# Linux kernel kfifo janitor patch - 9 November 2026

**Status:** Local patch produced and build-smoke-tested; **not emailed or merged upstream**.

The patch is authored under Charalampos Nadiotis (GitHub `n0711`) and is stored in `integrations/linux-kernel/0001-samples-kfifo-use-unsigned-formats-for-FIFO-counts.patch`. It is **not** a patch to the PELAOS TECH website. It is meant to apply in a Linux kernel tree.

## Change

- `samples/kfifo/bytestream-example.c`: two `printk()` calls use `%u` rather than `%d` for `unsigned int ret`.
- `samples/kfifo/inttype-example.c`: the same two replacements.
- No FIFO behavior, control flow, or API changes.

Upstream reference: Linux `torvalds/linux` tree at commit `7475ea8b58616b33c04afb976cf0fbaad9dd63fb` (Linux 7.4-rc2, 9 November 2026).

Original Git blob hashes, checked against the fetched upstream files:
- `samples/kfifo/bytestream-example.c`: `4ae29a12cc8a03aa2a5c55992f55eaed35982c70`.
- `samples/kfifo/inttype-example.c`: `e4f93317c5d049a337d39f2280c6fe52b45d3010`.

## Validation

**Verified on 9 November 2026**

- Verified the local original-source Git blob hashes match upstream.
- `git diff --check` and `git show --check` passed.
- `git apply --check` passed on the original source snapshot.
- Both modified samples compiled as out-of-tree kernel modules using the available Debian Linux **6.12.107+deb13-amd64** headers. This verifies compile compatibility with those headers only, not Linux 7.4.

**Outstanding before emailing**

- Apply the patch in a full Linux tree and run `scripts/checkpatch.pl`.
- Build with the matching upstream Linux 7.4-rc2 configuration, optionally run applicable kfifo tests.
- Recheck `scripts/get_maintainer.pl` output against the current `MAINTAINERS`.
- Review the patch text and the `Signed-off-by` line under the Developer's Certificate of Origin. The name/email must be correct.
- Verify the sending email account is configured for plain-text `git send-email`; never commit SMTP credentials.

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

After verification and explicit review of the dry-run recipients/message, remove `--dry-run` to submit by SMTP with `git send-email`. The KFIFO maintainer and Linux mailing-list addresses are a starting point, not a substitute for the current maintainer script. If using `git am`, use a clean branch to ensure the patch is not applied twice.

## Upstream references

- [Linux patch submission guide](https://docs.kernel.org/process/submitting-patches.html)
- [Linux patch submission checklist](https://docs.kernel.org/process/submit-checklist.html)
- [Kernel MAINTAINERS source](https://github.com/torvalds/linux/blob/master/MAINTAINERS)

Do not describe this as a merged or accepted kernel contribution until upstream acceptance is verified.
