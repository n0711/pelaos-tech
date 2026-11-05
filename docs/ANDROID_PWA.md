# PelaosTech mobile site and Android app path

## Phase 1 - portfolio PWA (implemented on the feature branch)

The published portfolio is still the Rust-generated static website with the existing Dart research calculator. The same output is installable in supported Android browsers using a web app manifest, 192/512 PNG launcher icons, and a scoped service worker. Navigation remains accessible and horizontally scrollable on narrow screens. No paid service, login, new backend, or analytics is added.

After the PR is merged and GitHub Pages finishes deploying, open **https://n0711.github.io/pelaos-tech/** on an Android device in Chrome. In the browser menu, use **Install app** or **Add to Home screen** when available. Confirm that the app opens in its own window, that navigation and the research slider work, and that pages visited or precached can be opened without network access. Installation availability depends on browser criteria and device settings, and has not yet been verified on a physical phone.

GitHub Pages is a **project site** with the path prefix `/pelaos-tech/`. Both the manifest and the service worker use the Rust compile-time `PELAOS_BASE_PATH` setting, so they also work at `/` if a root-domain Vercel deployment is later verified. The service worker handles same-origin GET requests only and caches public portfolio content - not external links or private data. Navigations use the network first, then cached pages; when offline, a missing page may fall back to the cached portfolio home. The service worker cache version should be bumped when its precache behavior changes.

Validation from the repository:
```sh
cargo test --locked
cargo run --release --locked -- dist
dart pub get
dart analyze --fatal-infos web/app.dart
dart compile js -O2 --csp -o dist/app.js web/app.dart
python3 scripts/check-pwa.py dist
```
The GitHub Pages and Vercel workflows run this check before publication. This verifies static assets but does **not** replace physical Android checks, Lighthouse checks, or an offline installation test.

## Phase 2 - Google Play Android (not released)

After the public web origin, offline behavior, accessibility and Android installation are verified, use a **Trusted Web Activity (TWA)** via [Bubblewrap](https://developer.android.com/develop/ui/views/layout/webapps/trusted-web-activities). This uses the installed website in a browser-backed Android app and avoids duplicating the Rust/Dart site. Before publishing:

1. Decide on a stable package/application ID and a permanent HTTPS origin. Prefer a controlled custom domain for long-term identity; the current GitHub Pages project path can remain a prototype.
2. Generate a signed Android App Bundle using an isolated Android build environment; secure the signing key and keep it outside the repository.
3. Host a valid `/.well-known/assetlinks.json` on the same **origin** to verify the Android package certificate with Digital Asset Links. On a GitHub Pages project path, origin-level control may be a blocker; switch to a verified custom domain rather than weakening verification.
4. Test on physical devices, including orientation, system back navigation, browser fallback, network changes, offline access, accessibility, and Android version compatibility.
5. Complete Play Console listing, screenshots, privacy disclosures, data-safety declarations, testing, review and any current eligibility requirements. Do not claim Play Store availability until the store listing is live.

No APK, AAB, Play listing, signed build, or Digital Asset Links assertion is created as part of Phase 1.

## Phase 3 - iPhone later

The existing mobile site should remain usable in Safari. A home-screen web app can be added using Safari's Share menu. An App Store app, if worthwhile, should be a separate product decision after Android validation. No iOS-specific native build or Apple developer account is required for Phase 1.
