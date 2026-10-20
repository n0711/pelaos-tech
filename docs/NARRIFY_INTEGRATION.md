# Narrify place-name layer: integration handoff

Narrify team: **Charalampos Nadiotis (n0711), Hussain Jamil and Nikitas Savva**.

The OpenPELAOS site now includes a credited project card and
`/cultural-storytelling-map/` page. The separate Narrify feature is preserved in
`integrations/narrify-place-names.patch` because the connected GitHub app can write
to n0711 repositories but rejected writes to
`hussainJamil24/cultural-storytelling-map` with HTTP 403, "Resource not accessible
by integration". No branch, commit or pull request was created in that team
repository.

## Target and changes

Target repository: https://github.com/hussainJamil24/cultural-storytelling-map  
Inspected base commit: `d8306ba6554320513a2b4bea93b8c97c2df3542b`

The patch adds a Stories / Place names map toggle; three individually sourced
pilot records (Nicosia, Anogyra, Lazanias); accent-insensitive current/historical
name search; origin evidence filters; visible source and coordinate attribution;
and nearby approved community stories within 2 km, clearly separated from
historical relationships. Draft or invalid records do not render. It preserves
alternative explanations and does not infer an origin when none is recorded.

It uses the existing React/Leaflet application and needs no new frontend
dependencies, API credentials or story database migration. The existing account
and moderation flows are preserved. Tests cover the data rules, card sources,
layer switching, empty filters, and story API failure. A minimal frontend CI
workflow is included for the team repository.

## Apply safely in a local Narrify checkout

First commit or stash any local work. Start a new review branch from the current
upstream main; the patch deliberately contains full-file context so it will stop
if those files have drifted.

```sh
git switch -c feat/openpelaos-place-names
git apply --check /path/to/narrify-place-names.patch
git apply /path/to/narrify-place-names.patch
cd frontend
npm ci
CI=true npm test -- --watchAll=false --runInBand
npm run build
```

Do not force an application if the check fails. Reconcile the patch against the
new source, then review the resulting diff and open a PR in the team repository.
The workflow file may require workflow-write permission when pushed.

## Verification actually performed

- 24 assertions executed against the prepared JavaScript helpers in a V8
  harness: source integrity, invalid/duplicate/draft exclusion, Greek/Turkish and
  historical search, competing explanations, origin-state filters, approved-only
  proximity results, invalid coordinates, ordering and non-mutation.
- Relative imports were resolved against the inspected upstream file tree.
- Every patch addition was reconstructed and compared to the prepared source.
- The seed summaries and coordinates were checked against the source links
  carried inside the patch.

A shell/build environment was unavailable in this session. Jest, the React
production build, Rust compilation and Dart compilation were not run locally.
The OpenPELAOS PR uses its existing Rust/Dart CI; that pipeline does not execute
the separate Narrify patch. Narrify React tests/build still need to run after
application in the team repository.

The screenshot is inspiration only; its raster, settlement boundaries and
language classifications are not copied. The patch's curation guide records the
pilot sources and separates compiler credit from the software team's credits.
