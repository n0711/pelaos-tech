#!/usr/bin/env python3
"""Validate generated PWA assets without adding third-party dependencies."""
import json
import pathlib
import struct
import sys
import zlib

def check_png(path: pathlib.Path, size: int) -> None:
    data = path.read_bytes()
    assert data.startswith(b"\x89PNG\r\n\x1a\n"), f"invalid PNG: {path}"
    assert struct.unpack(">II", data[16:24]) == (size, size), f"wrong dimensions: {path}"
    pos, idat = 8, bytearray()
    while pos < len(data):
        length = struct.unpack(">I", data[pos:pos + 4])[0]
        kind = data[pos + 4:pos + 8]
        payload = data[pos + 8:pos + 8 + length]
        stored_crc = struct.unpack(">I", data[pos + 8 + length:pos + 12 + length])[0]
        assert zlib.crc32(kind + payload) == stored_crc, f"bad PNG CRC: {path}"
        if kind == b"IDAT":
            idat.extend(payload)
        pos += length + 12
        if kind == b"IEND":
            break
    assert zlib.decompress(idat), f"bad PNG image data: {path}"

def main() -> None:
    root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else "dist")
    manifest = json.loads((root / "manifest.webmanifest").read_text(encoding="utf-8"))
    base = manifest["scope"]
    assert base.startswith("/") and base.endswith("/")
    assert manifest["id"] == base and manifest["start_url"] == base
    assert manifest["display"] == "standalone"
    icons = {icon["sizes"]: icon for icon in manifest["icons"]}
    for size in (192, 512):
        entry = icons[f"{size}x{size}"]
        assert entry["src"] == f"{base}icon-{size}.png"
        assert entry["type"] == "image/png"
        check_png(root / f"icon-{size}.png", size)

    worker = (root / "sw.js").read_text(encoding="utf-8")
    registration = (root / "pwa.js").read_text(encoding="utf-8")
    assert f"const BASE = '{base}'" in worker
    assert f"'{base}sw.js'" in registration
    assert f"'{base}'" in registration
    assert "url.origin !== self.location.origin" in worker
    for route in ("", "contact/", "services/", "research/", "vera-mot/", "ros3-limassol/",
                  "cultural-storytelling-map/", "riotuner/", "riotuner/testing/",
                  "riotuner/privacy/", "privacy/"):
        html = (root / route / "index.html").read_text(encoding="utf-8")
        assert f'href="{base}manifest.webmanifest"' in html
        assert f'src="{base}pwa.js"' in html
        assert '<meta name="viewport"' in html
    assert (root / "app.js").is_file(), "Dart must compile before this check"
    print(f"PWA validated: {base}, 11 routes, 2 PNG icons, worker and registration")

if __name__ == "__main__":
    main()
