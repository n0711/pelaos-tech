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

String _group(int n) {
  final s = n.toString();
  final b = StringBuffer();
  for (var i = 0; i < s.length; i++) {
    if (i > 0 && (s.length - i) % 3 == 0) b.write(',');
    b.write(s[i]);
  }
  return b.toString();
}

void main() {
  final host = _document.getElementById('pressure-live');
  if (host == null) return;
  // Same constants as the Rust generator (src/pages.rs), passed through data attributes.
  final rho = double.tryParse(host.getAttribute('data-rho') ?? '') ?? 1025;
  final g = double.tryParse(host.getAttribute('data-g') ?? '') ?? 9.80665;

  final label = _document.createElement('label')
    ..setAttribute('for', 'depth')
    ..textContent = 'Try a depth';
  final input = _document.createElement('input')
    ..id = 'depth'
    ..setAttribute('type', 'range')
    ..setAttribute('min', '0')
    ..setAttribute('max', '11000')
    ..setAttribute('step', '100')
    ..setAttribute('value', '4000');
  final output = _document.createElement('output')
    ..setAttribute('for', 'depth')
    ..setAttribute('aria-live', 'polite');

  void update() {
    final h = double.tryParse(input.value) ?? 0;
    final bar = rho * g * h / 1e5;
    final tonnes = rho * h * 0.01 / 1000; // 10 x 10 cm window
    output.textContent =
        '${_group(h.round())} m: ${bar.toStringAsFixed(0)} bar, ${tonnes.toStringAsFixed(1)} tonnes on a 10 × 10 cm window';
  }

  input.addEventListener('input', ((JSAny _) => update()).toJS);
  host
    ..append(label)
    ..append(input)
    ..append(output);
  update();
}
