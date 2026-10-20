# PELAOS — software projects & research

Live at **https://n0711.github.io/pelaos-tech/** (GitHub Pages).

| Part | Language | Role |
|---|---|---|
| `src/` | Rust | Renders every page to complete HTML at build time: content, layout, JSON-LD, sitemap. GitHub Pages can't run a server, so the "backend" runs in the build. |
| `web/app.dart` | Dart | Compiled to `app.js`. Adds the live depth slider on /research/. Pages work fully without it. |
| `.github/workflows/pages.yml` | | Tests and builds pull requests; deploys on every push to `main`. |

## Pages

- `/`: project portfolio with RioTuner, cysk8clips and LearnTwin
- `/research/`: pressure at depth, ROV vs AUV, navigation without GPS (NOAA sources)
- `/riotuner/`: RioTuner overview, pre-release status and synthetic benchmark limits
- `/riotuner/testing/`: practical device-testing guide and feedback channels
- `/riotuner/privacy/`: RioTuner privacy policy (the URL to enter in Google Play Console)
- `/privacy/`: website privacy

## GitHub Pages compliance

Checked against the [GitHub Pages docs](https://docs.github.com/en/pages) on 18 October 2026.

| Rule | How this site meets it |
|---|---|
| GitHub Free publishes Pages only from public repositories | The repository is public. It contains only the site's source, no secrets. |
| Not for running an online business, e-commerce or SaaS | Project and research information plus a privacy policy. Nothing is sold and no transactions happen on the site. |
| No sensitive transactions (passwords, credit cards) | No forms or inputs that send data. The CSP sets `form-action 'none'`, and a test fails the build if a `<form>` appears. |
| Site ≤ 1 GB, repository ≤ 1 GB recommended, deploy < 10 min, soft 100 GB/month bandwidth | About 90 KB in total, and the workflow jobs time out at 10 minutes. |
| HTTPS, no mixed content | github.io is served over HTTPS with "Enforce HTTPS" on. A test fails the build on any `http://` asset or link. |
| Visitors' IP addresses are logged by GitHub | Disclosed on `/privacy/`. |
| Custom workflow: `pages: write` + `id-token: write`, `github-pages` environment | Granted only to the deploy job. The build job only reads. Actions are pinned to release commits. |
| GitHub Terms of Service and Acceptable Use Policies | Original content with cited sources. |

## Build locally

Needs a stable Rust toolchain and the Dart SDK 3.5 or newer. Neither the Rust
generator nor the Dart script pulls in third-party packages.

```powershell
cargo test
cargo run -- dist
dart pub get
dart analyze --fatal-infos web/app.dart
dart compile js -O2 --csp -o dist/app.js web/app.dart
```

Pull requests run the Rust tests, render the site, analyze the Dart enhancement
(including informational diagnostics), and compile it before deployment is eligible.
Only pushes to `main` and manually triggered workflows deploy the site.

### Output directory safety

The Rust generator accepts a new or empty output directory. Later builds replace
that directory only when it contains the generator's `.pelaos-site-output` marker.
The whole marked directory is generated output; keep source files elsewhere and
do not add the marker manually. Symbolic links and `..` in output paths are rejected.
A complete build is staged before the previous build is replaced.

An older, unmarked `dist` is deliberately refused. Use a fresh output path (for
example `cargo run -- dist-new`), or inspect and manually remove the old generated
directory before rebuilding. Never use the repository or a source directory as output.

## Moving to pelaos-tech.com later

Change `SITE` and `BASE` in `src/pages.rs` (`https://pelaos-tech.com` and `""`), add a `CNAME` file containing `pelaos-tech.com` to the output, and set the custom domain in the repository's Pages settings. `robots.txt` and `llms.txt` take effect only at a domain root, so they start working after that move.

© 2026 PelaosTech.

## Product documentation

- [Commercial priorities and decision record](docs/PRODUCT_DECISIONS.md)
- [Fixed-scope website service](docs/SERVICE_OFFER.md)

The Pages site is an informational portfolio. It has no checkout or sales form.
Commercial offer documents live in the repository, not the generated Pages site.
A custom domain does not remove the hosting provider's usage restrictions.
