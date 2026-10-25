// Progressive enhancement for /research/: a live depth slider next to the pressure table.
// The page is complete without it; the Rust generator renders the same figures at build
// time. Uses only dart:js_interop, so there are no package dependencies.

import 'dart:js_interop';

@JS('document')
external _Document get _document;

extension type _Document._(JSObject _) implements JSObject {
  external _Element? getElementById(String id);
  external _Element createElement(String tag);
}

extension type _Element._(JSObject _) implements JSObject {
  external set textContent(String value);
  external set id(String value);
  external String get value;
  external String? getAttribute(String name);
  external void setAttribute(String name, String value);
  external void append(_Element child);
  external void addEventListener(String type, JSFunction listener);
}

// Match the Rust table's half-up rounding at values such as 5.125 tonnes.
String _fixed2(num value) => ((value * 100).round() / 100).toStringAsFixed(2);

void main() {
  final host = _document.getElementById('pressure-live');
  if (host == null) return;
  // Same constants as the Rust generator (src/pages.rs), passed through data attributes.
  final rho = double.tryParse(host.getAttribute('data-rho') ?? '');
  final g = double.tryParse(host.getAttribute('data-g') ?? '');
  final maxDepth = int.tryParse(host.getAttribute('data-max-depth') ?? '');
  final initialDepth =
      int.tryParse(host.getAttribute('data-initial-depth') ?? '');
  if (rho == null ||
      !rho.isFinite ||
      rho <= 0 ||
      g == null ||
      !g.isFinite ||
      g <= 0 ||
      maxDepth == null ||
      maxDepth <= 0 ||
      initialDepth == null) return;

  final label = _document.createElement('label')
    ..setAttribute('for', 'depth')
    ..textContent = 'Depth (0–$maxDepth m)';
  final input = _document.createElement('input')
    ..id = 'depth'
    ..setAttribute('type', 'range')
    ..setAttribute('min', '0')
    ..setAttribute('max', '$maxDepth')
    ..setAttribute('step', '1')
    ..setAttribute('value', '${initialDepth.clamp(0, maxDepth)}')
    ..setAttribute('aria-describedby', 'pressure-model');
  final output = _document.createElement('output')
    ..setAttribute('for', 'depth')
    ..setAttribute('aria-live', 'polite');

  void update() {
    final parsed = double.tryParse(input.value);
    final h =
        (parsed != null && parsed.isFinite ? parsed : 0.0).clamp(0, maxDepth);
    final bar = rho * g * h / 1e5;
    final tonnes = rho * h * 0.01 / 1000; // 10 x 10 cm window
    input.setAttribute('aria-valuetext', '${h.round()} m');
    output.textContent =
        '${h.round()} m: ${_fixed2(bar)} bar above surface, ${_fixed2(tonnes)} tonnes-force on a 10 × 10 cm window';
  }

  input.addEventListener('input', ((JSAny _) => update()).toJS);
  host
    ..append(label)
    ..append(input)
    ..append(output);
  update();
}
