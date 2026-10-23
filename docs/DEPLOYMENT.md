# PELAOS deployment

## Publication status

The existing published address is `https://n0711.github.io/pelaos-tech/`.
Vercel support is prepared in this repository; no verified Vercel production
address is recorded yet. Use the address returned by the connected Vercel
project after a successful production deployment.

## One web deployment, three repository references

Deploy `n0711/pelaos-tech` to host the portfolio and RioTuner's public pages.
`n0711/riotuner` contains the native Rust/Flutter Android app and C# desktop app;
it is not a Flutter Web project. `n0711/n0711` is the GitHub profile README.
Their README links should point to the published PELAOS pages once verified.

| Resource | Path on the Vercel production origin |
| --- | --- |
| Portfolio | `/` |
| Research | `/research/` |
| Narrify | `/cultural-storytelling-map/` |
| RioTuner | `/riotuner/` |
| Testing guide | `/riotuner/testing/` |
| App privacy | `/riotuner/privacy/` |
| Website privacy | `/privacy/` |
| Sitemap | `/sitemap.xml` |

## Vercel settings

Import `n0711/pelaos-tech` with root directory `.`. `vercel.json` supplies the
Other framework preset, `bash scripts/build-vercel.sh`, `dist` output and
trailing-slash routes. No application server or Vercel Function is required.

Enable access to system environment variables. The build reads the production
origin from `VERCEL_PROJECT_PRODUCTION_URL`, even for previews. An explicit
`PELAOS_SITE_URL=https://your-production-domain` overrides it. The value must
have no trailing slash or path. The script sets `PELAOS_BASE_PATH` to an empty
string and `PELAOS_HOSTING=vercel`.

Do not use a preview `VERCEL_URL` as the production canonical. Project or
custom-domain changes require a new deployment to rebuild the HTML,
structured data and sitemap.

The build obtains Rust if needed, uses checksum-verified Dart 3.13.5, runs the
generator tests, renders all pages and compiles the progressive enhancement.
Tool downloads are kept outside `dist` and ignored by Git.

## GitHub Pages settings

With no deployment overrides, the generator keeps:

```text
PELAOS_SITE_URL=https://n0711.github.io/pelaos-tech
PELAOS_BASE_PATH=/pelaos-tech
PELAOS_HOSTING=github-pages
```

The Pages workflow retains its existing deployment behaviour. Vercel's build
overrides apply only to the Vercel build process.

## Verify publication

1. Confirm Vercel reports the production deployment as Ready and record its actual domain.
2. Open the home, research, RioTuner, testing, app-privacy and website-privacy paths directly. Check the research slider and favicon load from `/`.
3. Check a missing page returns HTTP 404, and `sitemap.xml` and `robots.txt` use the production origin.
4. Inspect canonical and Open Graph URLs. They should use the production origin without `/pelaos-tech/` on Vercel.
5. Update the PELAOS, RioTuner and n0711 READMEs with the verified origin. Update the privacy-policy URL in Play Console separately when store setup is performed.

Successful site deployment does not build an Android APK or establish phone
accuracy. RioTuner release validation belongs to its native-app repository.

## Official references

- [Vercel build and output-directory settings](https://vercel.com/docs/builds/configure-a-build)
- [Vercel JSON configuration](https://vercel.com/docs/project-configuration/vercel-json)
- [Vercel system environment variables](https://vercel.com/docs/environment-variables/system-environment-variables)
- [Vercel Privacy Notice](https://vercel.com/legal/privacy-notice)
