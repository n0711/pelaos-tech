# PELAOS SEO: launch and measurement checklist

Last reviewed: 27 October 2026. This is a repository-level plan, **not evidence of indexing, search traffic or production health**.

## Intended search pages and queries

| Canonical path | Primary reader intent | Truthful content focus |
| --- | --- | --- |
| `/` | PELAOS / PelaosTech, software projects Cyprus | Independent software projects and robotics research portfolio |
| `/services/` | small business website development Cyprus; one-page website Cyprus | Proposed fixed-scope pilot; no open booking, payment or ranking guarantees |
| `/research/` | marine robotics depth calculator; underwater pressure | 0–500 m illustrative calculator with NOAA sources; not sensor-derived |
| `/ros3-limassol/` | ROS3-Limassol telemetry; ROS 2 intermittent connectivity | Receiver-arrival evidence and synthetic validation limits |
| `/riotuner/` | offline instrument tuner Android; bouzouki tuner | Pre-release app; synthetic benchmarks only |
| `/cultural-storytelling-map/` | Narrify cultural storytelling map Cyprus | Team credits and scoped pilot content |

Use the other existing information and privacy pages for navigation and compliance. Do not create shallow duplicate pages for slight keyword variations.

## Before requesting indexing

1. Choose one verified **production** origin. The existing documented publication is `https://n0711.github.io/pelaos-tech/`; the Vercel URL is not verified in this repository. Do not use a Vercel preview URL as canonical.
2. After deploying, inspect the live HTML at `/`, `/services/`, `/research/`, and `/riotuner/` and ensure the HTTP status is 200. The new page must be reachable from the home page and navigation.
3. Check each page's `<title>`, unique meta description, canonical URL, Open Graph URL, valid JSON-LD and exactly one H1. The `/404.html` page should remain `noindex` and excluded from the sitemap.
4. Inspect `sitemap.xml`: all URL locations must use the selected production origin, include `/services/` and omit the 404 page. Verify that `robots.txt` points to the same sitemap.
5. **GitHub Pages project-path caveat:** `/pelaos-tech/robots.txt` is not a robots policy for `n0711.github.io` because robots directives are discovered at the host root. The sitemap can still be submitted directly in Search Console. The root-level `robots.txt` emitted on Vercel is effective when served at `/robots.txt`.
6. If GitHub Pages and Vercel serve the same content concurrently, decide which is authoritative. Where redirects cannot be set, put cross-host canonical URLs pointing to the authoritative host; avoid two self-canonical production hosts indefinitely. Do not switch the configured canonical before the preferred domain is serving the site.
7. Confirm GitHub Pages or Vercel published the commit by checking deployment status and loading the new page. Passing CI alone is not a live-site audit.

## After deploying

- Verify ownership of the exact site property in [Google Search Console](https://search.google.com/search-console/about); use a URL-prefix property for `https://n0711.github.io/pelaos-tech/` if staying on GitHub Pages.
- Submit the **live**, absolute sitemap URL. Inspect the homepage, service pilot, ROS3 and research pages. Use URL Inspection to see whether Google can fetch them, but do not claim that a submitted URL is already indexed.
- Record a baseline for branded searches (`PELAOS`, `PelaosTech`) and page-specific queries. Track search impressions, clicks, CTR, average position and indexed-page coverage monthly. Filter by country Cyprus where useful, but do not infer searches or conversions from limited data.
- Add one clearly sourced case study when a real client pilot is completed and permission for any images and quotes has been obtained. Do not publish invented testimonials, addresses, reviews, awards, prices or measured results.
- Add descriptive social-preview image metadata only once an actual hosted, crawler-compatible preview image exists. Do not point `og:image` to a non-existent asset.

## Repeatable engineering checks

```sh
cargo test --locked
cargo run --release --locked -- dist
# Replace with the actual selected public sitemap, after publishing:
curl -fsS https://n0711.github.io/pelaos-tech/sitemap.xml
```

Tests verify source-generated page metadata and sitemap coverage. Live HTTP status, search indexing, Core Web Vitals, traffic and keyword rankings must be verified separately.

Official guidance: [Google SEO Starter Guide](https://developers.google.com/search/docs/fundamentals/seo-starter-guide), [Google crawling and indexing](https://developers.google.com/search/docs/crawling-indexing/overview), [robots.txt specification](https://developers.google.com/search/docs/crawling-indexing/robots/intro).
