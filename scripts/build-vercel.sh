#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")/.."

# This builds static pages; Vercel never runs the Rust generator per request.
if [[ -z "${PELAOS_SITE_URL:-}" ]]; then
  if [[ -z "${VERCEL_PROJECT_PRODUCTION_URL:-}" ]]; then
    echo 'Set PELAOS_SITE_URL to the HTTPS production origin, or enable Vercel system environment variables.' >&2
    exit 1
  fi
  export PELAOS_SITE_URL="https://${VERCEL_PROJECT_PRODUCTION_URL}"
fi
export PELAOS_BASE_PATH=''
export PELAOS_HOSTING='vercel'

tools="$PWD/.build-tools"
mkdir -p "$tools"
if ! command -v cargo >/dev/null 2>&1; then
  curl --fail --silent --show-error --location --retry 3 https://sh.rustup.rs -o "$tools/rustup-init.sh"
  export CARGO_HOME="$tools/cargo" RUSTUP_HOME="$tools/rustup"
  sh "$tools/rustup-init.sh" -y --profile minimal --default-toolchain stable --no-modify-path
  export PATH="$CARGO_HOME/bin:$PATH"
fi

dart_version='3.13.5'
if command -v dart >/dev/null 2>&1 && dart --version 2>&1 | grep -Fq "Dart SDK version: $dart_version "; then
  dart_bin="$(command -v dart)"
else
  dart_bin="$tools/dart-$dart_version/dart-sdk/bin/dart"
  if [[ ! -x "$dart_bin" ]]; then
    archive='dartsdk-linux-x64-release.zip'
    download="$tools/dart-$dart_version"
    mkdir -p "$download"
    sdk_url="https://storage.googleapis.com/dart-archive/channels/stable/release/$dart_version/sdk/$archive"
    curl --fail --silent --show-error --location --retry 3 "$sdk_url" -o "$download/$archive"
    curl --fail --silent --show-error --location --retry 3 "$sdk_url.sha256sum" -o "$download/sdk.sha256"
    (cd "$download" && sha256sum --check sdk.sha256 && unzip -q -o "$archive")
    rm "$download/$archive" "$download/sdk.sha256"
  fi
fi

cargo test --locked
cargo run --release --locked -- dist
"$dart_bin" pub get
"$dart_bin" analyze --fatal-infos web/app.dart
"$dart_bin" compile js -O2 --csp -o dist/app.js web/app.dart
rm -f dist/app.js.deps dist/app.js.map
