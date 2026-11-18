#!/usr/bin/env python3
"""Audit generated SEO metadata offline (not live search-engine indexing)."""
import json
import sys
from html.parser import HTMLParser
from pathlib import Path
from urllib.parse import urlsplit
from xml.etree import ElementTree as ET


class Page(HTMLParser):
    def __init__(self, source):
        super().__init__(convert_charrefs=True)
        self.title = []
        self._inside_title = False
        self._inside_json = False
        self._json_parts = []
        self.json_docs = []
        self.meta = {}
        self.canonicals = []
        self.feed(source)

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == "title":
            self._inside_title = True
        elif tag == "meta":
            key = attrs.get("name") or attrs.get("property")
            if key:
                self.meta.setdefault(key, []).append(attrs.get("content", ""))
        elif tag == "link" and attrs.get("rel") == "canonical":
            self.canonicals.append(attrs.get("href", ""))
        elif tag == "script" and attrs.get("type") == "application/ld+json":
            self._inside_json = True
            self._json_parts = []

    def handle_data(self, data):
        if self._inside_title:
            self.title.append(data)
        if self._inside_json:
            self._json_parts.append(data)

    def handle_endtag(self, tag):
        if tag == "title":
            self._inside_title = False
        elif tag == "script" and self._inside_json:
            self.json_docs.append(json.loads("".join(self._json_parts)))
            self._inside_json = False

    def one_meta(self, key):
        values = self.meta.get(key, [])
        check(len(values) == 1 and bool(values[0]), f"expected one {key}: {values}")
        return values[0]


def check(condition, message):
    if not condition:
        raise AssertionError(message)


def audit(output):
    check((output / "index.html").is_file(), "missing homepage")
    home = Page((output / "index.html").read_text(encoding="utf-8"))
    check(len(home.canonicals) == 1, "homepage must have one canonical")
    origin = home.canonicals[0]
    check(origin.startswith("https://") and origin.endswith("/"), "homepage canonical must be an HTTPS directory URL")
    check(not urlsplit(origin).query and not urlsplit(origin).fragment, "unexpected canonical query/fragment")
    check((output / "icon-512.png").is_file(), "Open Graph logo must exist")

    page_files = sorted(output.rglob("index.html"))
    urls = set()
    # Enforce site-wide uniqueness in addition to per-page metadata validity.
    descriptions = set()
    titles = set()
    for path in page_files:
        relative = path.relative_to(output).as_posix()
        suffix = relative[:-len("index.html")]
        expected = origin + suffix
        doc = Page(path.read_text(encoding="utf-8"))
        check(len(doc.canonicals) == 1 and doc.canonicals[0] == expected,
              f"{relative}: canonical mismatch")
        description = doc.one_meta("description")
        check(description not in descriptions, f"{relative}: duplicate description")
        descriptions.add(description)
        title = "".join(doc.title).strip()
        check(bool(title), f"{relative}: empty title")
        check(title not in titles, f"{relative}: duplicate title")
        titles.add(title)
        check(doc.one_meta("og:url") == expected, f"{relative}: Open Graph URL mismatch")
        check(doc.one_meta("og:title") == title, f"{relative}: Open Graph title mismatch")
        check(doc.one_meta("twitter:title") == title, f"{relative}: Twitter title mismatch")
        check(doc.one_meta("og:description") == description,
              f"{relative}: Open Graph description mismatch")
        check(doc.one_meta("og:image") == origin + "icon-512.png",
              f"{relative}: Open Graph image missing or noncanonical")
        check(doc.one_meta("twitter:image") == origin + "icon-512.png",
              f"{relative}: Twitter image mismatch")
        check("noindex" not in doc.meta.get("robots", []), f"{relative}: unexpected noindex")
        check(len(doc.json_docs) == 1, f"{relative}: expected one JSON-LD graph")
        graph = doc.json_docs[0].get("@graph", [])
        check(any(node.get("@type") in ("WebPage", "ContactPage") and node.get("url") == expected
                  for node in graph), f"{relative}: wrong JSON-LD page URL")
        check(any(node.get("@type") == "Organization"
                  and node.get("logo") == origin + "icon-512.png" for node in graph),
              f"{relative}: organization logo mismatch")
        urls.add(expected)

    missing = Page((output / "404.html").read_text(encoding="utf-8"))
    check(missing.one_meta("robots") == "noindex", "404 must have robots noindex")

    root = ET.parse(output / "sitemap.xml").getroot()
    ns = "{http://www.sitemaps.org/schemas/sitemap/0.9}"
    listed = [node.text for node in root.findall(f"{ns}url/{ns}loc")]
    check(len(listed) == len(set(listed)), "sitemap contains duplicate URLs")
    check(set(listed) == urls, "sitemap and canonical page set differ")
    check(not root.findall(f".//{ns}lastmod"), "unverified global lastmod dates were added")
    robots = (output / "robots.txt").read_text(encoding="utf-8")
    check(f"Sitemap: {origin}sitemap.xml" in robots, "robots.txt sitemap URL mismatch")
    check(not any("404" in url for url in urls), "404 is indexable")
    print(f"SEO audit passed: {len(urls)} canonical HTML pages, JSON-LD, social metadata and sitemap agree.")
    print("Source audit only: live HTTP status, Search Console indexing and rankings remain unverified.")


if __name__ == "__main__":
    try:
        check(len(sys.argv) == 2, "usage: check-seo.py <generated-site-directory>")
        audit(Path(sys.argv[1]))
    except (AssertionError, OSError, ValueError, ET.ParseError) as exc:
        print(f"SEO audit FAILED: {exc}", file=sys.stderr)
        sys.exit(1)
