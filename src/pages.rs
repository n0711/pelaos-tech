//! Page content, layout, structured data, sitemap. Tests at the bottom enforce the
//! GitHub Pages and SEO rules the site relies on.

/// Canonical origin + base path, without a trailing slash. Change these two (and add a
/// CNAME file) when the site moves to its own domain.
pub const SITE: &str = "https://n0711.github.io/pelaos-tech";
pub const BASE: &str = "/pelaos-tech";
/// Shown in the footer, JSON-LD `dateModified` and sitemap `lastmod`.
pub const UPDATED: &str = "2026-10-18";

/// Seawater density and standard gravity used for every pressure figure on the site.
pub const RHO: f64 = 1025.0;
pub const G: f64 = 9.80665;

const NOAA_PRESSURE: &str = "https://oceanservice.noaa.gov/facts/pressure.html";
const NOAA_AUV_ROV: &str = "https://oceanservice.noaa.gov/facts/auv-rov.html";
const NOAA_NAVIGATION: &str = "https://oceanexplorer.noaa.gov/expedition-feature/technology-development-partnerships-ex1904-logs-july29/";
const GITHUB_PAGES_PRIVACY: &str = "https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages";
const CONTACT: &str = "https://github.com/n0711/pelaos-tech/issues";

pub struct File {
    pub path: String,
    pub contents: String,
}

struct Page {
    /// Path below BASE: "" for home, "research/" etc.
    slug: &'static str,
    title: &'static str,
    description: &'static str,
    body: String,
    crumbs: Vec<(&'static str, &'static str)>,
    extra_ld: Vec<String>,
    script: bool,
    noindex: bool,
}

fn href(slug: &str) -> String {
    format!("{BASE}/{slug}")
}

fn url(slug: &str) -> String {
    format!("{SITE}/{slug}")
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// JSON string literal.
fn js(s: &str) -> String {
    let mut o = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => o.push_str("\\\""),
            '\\' => o.push_str("\\\\"),
            '\n' => o.push_str("\\n"),
            '<' => o.push_str("\\u003c"),
            c => o.push(c),
        }
    }
    o.push('"');
    o
}

fn group(n: u64) -> String {
    let s = n.to_string();
    let mut o = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i) % 3 == 0 {
            o.push(',');
        }
        o.push(c);
    }
    o
}

/// Gauge pressure (above surface pressure) at depth `h` metres, in bar.
pub fn gauge_bar(h: f64) -> f64 {
    RHO * G * h / 1e5
}

/// Force on a 10 x 10 cm window at depth `h`, in tonnes-force.
pub fn window_tonnes(h: f64) -> f64 {
    RHO * h * 0.01 / 1000.0
}

const CSS: &str = r#"
:root{--bg:#f6f8fb;--fg:#0b1424;--muted:#4a5870;--card:#fff;--line:#d7deea;--accent:#0a7ea4}
@media (prefers-color-scheme:dark){:root{--bg:#070c18;--fg:#e9eef7;--muted:#9aa6bb;--card:#101829;--line:#26324a;--accent:#39d0f2}}
*{box-sizing:border-box}
body{margin:0;background:var(--bg);color:var(--fg);font:17px/1.6 system-ui,-apple-system,"Segoe UI",Roboto,sans-serif}
a{color:var(--accent)}
.skip{position:absolute;left:-999px}.skip:focus{left:16px;top:8px;background:var(--card);padding:8px}
header,main,footer{max-width:760px;margin:0 auto;padding:0 16px}
header{display:flex;flex-wrap:wrap;align-items:center;justify-content:space-between;gap:8px;padding-top:16px;padding-bottom:8px}
.brand{font-weight:800;font-size:20px;text-decoration:none;color:var(--fg)}
nav a{display:inline-block;min-height:44px;line-height:44px;padding:0 10px;text-decoration:none}
nav a[aria-current]{font-weight:700;color:var(--fg)}
h1{font-size:34px;line-height:1.2;margin:24px 0 8px}
h2{font-size:23px;margin:32px 0 8px}
.lead{font-size:19px;color:var(--muted)}
.card{background:var(--card);border:1px solid var(--line);border-radius:12px;padding:16px;margin:16px 0}
table{border-collapse:collapse;width:100%;font-variant-numeric:tabular-nums}
th,td{text-align:right;padding:6px 8px;border-bottom:1px solid var(--line)}
th:first-child,td:first-child{text-align:left}
dt{font-weight:700;margin-top:16px}dd{margin:4px 0 0}
#pressure-live{display:grid;gap:8px;margin-top:12px}
#pressure-live input{width:100%;min-height:44px}
small,.muted{color:var(--muted)}
footer{border-top:1px solid var(--line);margin-top:48px;padding-top:16px;padding-bottom:32px;font-size:15px;color:var(--muted)}
footer a{display:inline-block;min-height:44px;line-height:44px;margin-right:12px}
"#;

fn layout(p: &Page) -> String {
    let canonical = url(p.slug);
    let nav = [("", "Home"), ("research/", "Research"), ("riotuner/", "RioTuner")]
        .iter()
        .map(|(slug, label)| {
            let current = if *slug == p.slug { " aria-current=\"page\"" } else { "" };
            format!("<a href=\"{}\"{current}>{label}</a>", href(slug))
        })
        .collect::<Vec<_>>()
        .join("");

    let mut crumbs = vec![("", "Home")];
    crumbs.extend(p.crumbs.iter().copied());
    let crumb_ld = crumbs
        .iter()
        .enumerate()
        .map(|(i, (slug, name))| {
            format!(
                "{{\"@type\":\"ListItem\",\"position\":{},\"name\":{},\"item\":{}}}",
                i + 1,
                js(name),
                js(&url(slug))
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let mut graph = vec![
        format!(
            "{{\"@type\":\"Organization\",\"@id\":{},\"name\":\"PelaosTech\",\"url\":{}}}",
            js(&format!("{SITE}/#org")),
            js(&url(""))
        ),
        format!(
            "{{\"@type\":\"WebSite\",\"@id\":{},\"name\":\"PelaosTech\",\"url\":{},\"publisher\":{{\"@id\":{}}}}}",
            js(&format!("{SITE}/#site")),
            js(&url("")),
            js(&format!("{SITE}/#org"))
        ),
        format!(
            "{{\"@type\":\"WebPage\",\"name\":{},\"url\":{},\"description\":{},\"dateModified\":\"{UPDATED}\",\"isPartOf\":{{\"@id\":{}}}}}",
            js(p.title),
            js(&canonical),
            js(p.description),
            js(&format!("{SITE}/#site"))
        ),
        format!("{{\"@type\":\"BreadcrumbList\",\"itemListElement\":[{crumb_ld}]}}"),
    ];
    graph.extend(p.extra_ld.iter().cloned());
    let ld = format!("{{\"@context\":\"https://schema.org\",\"@graph\":[{}]}}", graph.join(","));

    let robots = if p.noindex { "<meta name=\"robots\" content=\"noindex\">\n" } else { "" };
    let script = if p.script { format!("<script src=\"{BASE}/app.js\" defer></script>\n") } else { String::new() };
    format!(
        r##"<!doctype html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta http-equiv="Content-Security-Policy" content="default-src 'self'; script-src 'self'; style-src 'unsafe-inline'; img-src 'self' data:; object-src 'none'; base-uri 'none'; form-action 'none'">
<meta name="referrer" content="strict-origin-when-cross-origin">
<title>{title}</title>
<meta name="description" content="{desc}">
{robots}<link rel="canonical" href="{canonical}">
<link rel="icon" href="{BASE}/favicon.svg" type="image/svg+xml">
<meta name="color-scheme" content="light dark">
<meta property="og:type" content="website">
<meta property="og:site_name" content="PelaosTech">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{desc}">
<meta property="og:url" content="{canonical}">
<meta name="twitter:card" content="summary">
<style>{css}</style>
<script type="application/ld+json">{ld}</script>
{script}</head>
<body>
<a class="skip" href="#main">Skip to content</a>
<header><a class="brand" href="{home}">PelaosTech</a><nav aria-label="Main">{nav}</nav></header>
<main id="main">
{body}
</main>
<footer>
<a href="{research}">Research</a><a href="{riotuner}">RioTuner</a><a href="{privacy}">Website privacy</a><a href="{CONTACT}">Contact</a>
<p>© 2026 PelaosTech. Last updated <time datetime="{UPDATED}">18 October 2026</time>. Hosted on GitHub Pages.</p>
</footer>
</body>
</html>
"##,
        title = esc(p.title),
        desc = esc(p.description),
        css = CSS.trim(),
        home = href(""),
        research = href("research/"),
        riotuner = href("riotuner/"),
        privacy = href("privacy/"),
        body = p.body.trim(),
    )
}

fn home() -> Page {
    let body = format!(
        r#"
<h1>PelaosTech</h1>
<p class="lead">Marine robotics research and precision software.</p>
<section class="card">
<h2>Deep-sea robotics research</h2>
<p>A research concept for an autonomous, deep-submergence remotely operated vehicle (ROV) for extreme underwater and volcanic environments: high pressure, poor visibility, unstable terrain and no GPS. The project is at the research stage.</p>
<p><a href="{research}">The numbers behind deep-sea design</a>: pressure at depth, and how robots navigate without GPS.</p>
</section>
<section class="card">
<h2>RioTuner</h2>
<p>A precision tuner for guitar, bass, ukulele and bouzouki. It measures every overtone of a string and fits a stiff-string model, so readings stay within a fraction of a cent, even on a phone microphone.</p>
<p><a href="{riotuner}">About RioTuner</a></p>
</section>
"#,
        research = href("research/"),
        riotuner = href("riotuner/"),
    );
    Page {
        slug: "",
        title: "PelaosTech: marine robotics and precision software",
        description: "PelaosTech researches deep-sea robotics and builds precision software, including RioTuner, a sub-cent accurate instrument tuner.",
        body,
        crumbs: vec![],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn research() -> Page {
    let rows = [10.0, 100.0, 1000.0, 2000.0, 4000.0, 6000.0, 11000.0]
        .iter()
        .map(|&h| {
            format!(
                "<tr><td>{} m</td><td>{:.0}</td><td>{:.1}</td></tr>",
                group(h as u64),
                gauge_bar(h),
                window_tonnes(h)
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let bar4000 = gauge_bar(4000.0);
    let t4000 = window_tonnes(4000.0);
    let faq = [
        (
            "What is the difference between an ROV and an AUV?",
            "An ROV (remotely operated vehicle) is an uncrewed underwater robot connected to a ship by cables, through which an operator controls it. An AUV (autonomous underwater vehicle) has no tether and follows a pre-programmed mission.".to_string(),
        ),
        (
            "How does an underwater robot navigate without GPS?",
            "GPS does not work underwater, because the ocean absorbs the radio signals. Vehicles combine dead reckoning (inertial sensors, compass, depth and speed) with acoustic positioning against beacons or a surface vessel, and correct their position with GPS when they surface.".to_string(),
        ),
        (
            "How much pressure is there at 4,000 metres?",
            format!(
                "About {bar4000:.0} bar above surface pressure, roughly 400 times the air pressure at sea level. On a 10 x 10 cm window that is a force of about {t4000:.0} tonnes."
            ),
        ),
    ];
    let faq_html = faq
        .iter()
        .map(|(q, a)| format!("<dt>{}</dt>\n<dd>{}</dd>", esc(q), esc(a)))
        .collect::<Vec<_>>()
        .join("\n");
    let faq_ld = format!(
        "{{\"@type\":\"FAQPage\",\"mainEntity\":[{}]}}",
        faq.iter()
            .map(|(q, a)| format!(
                "{{\"@type\":\"Question\",\"name\":{},\"acceptedAnswer\":{{\"@type\":\"Answer\",\"text\":{}}}}}",
                js(q),
                js(a)
            ))
            .collect::<Vec<_>>()
            .join(",")
    );
    let body = format!(
        r#"
<h1>Deep-sea robotics: the numbers</h1>
<p class="lead">Why deep-sea vehicles are hard to build, in figures, with sources.</p>

<h2>Pressure</h2>
<p>NOAA: <q>For every 33 feet (10.06 meters) you go down, the pressure increases by one atmosphere.</q> (<a href="{NOAA_PRESSURE}">NOAA Ocean Service</a>)</p>
<div class="card">
<table>
<caption class="muted">Pressure above surface pressure, and the force on a 10 × 10 cm viewport</caption>
<thead><tr><th scope="col">Depth</th><th scope="col">Pressure (bar)</th><th scope="col">Force on window (tonnes)</th></tr></thead>
<tbody>
{rows}
</tbody>
</table>
<div id="pressure-live" data-rho="{RHO}" data-g="{G}"></div>
<p><small>Computed with seawater density {RHO} kg/m³ and g = {G} m/s². Real seawater gets slightly denser with depth, so true pressures are a little higher (about 1–2% at 4,000 m).</small></p>
</div>

<h2>Navigation without GPS</h2>
<p><q>GPS does not work underwater</q>, so a submerged vehicle's position estimate drifts until it can surface for a new fix (<a href="{NOAA_NAVIGATION}">NOAA Ocean Exploration</a>). Deep vehicles therefore combine dead reckoning (inertial sensors, compass, depth and speed) with acoustic positioning, where sound pulses between the vehicle and beacons or a surface vessel give its range and bearing.</p>

<h2>FAQ</h2>
<dl>
{faq_html}
</dl>
<p><small>ROV and AUV definitions: <a href="{NOAA_AUV_ROV}">NOAA Ocean Service</a>.</small></p>
"#
    );
    Page {
        slug: "research/",
        title: "Deep-sea robotics: pressure and navigation | PelaosTech",
        description: "Pressure at depth (about 402 bar at 4,000 m), ROV vs AUV, and how underwater robots navigate without GPS. Figures with NOAA sources.",
        body,
        crumbs: vec![("research/", "Research")],
        extra_ld: vec![faq_ld],
        script: true,
        noindex: false,
    }
}

fn riotuner() -> Page {
    let body = format!(
        r#"
<h1>RioTuner</h1>
<p class="lead">A precision tuner for guitar, bass, ukulele and bouzouki, for Android. Currently in testing; a Google Play release is planned.</p>

<h2>How it measures</h2>
<p>Most tuner apps time the waveform of a string. RioTuner measures each overtone separately, from the way its phase advances over time, and fits a stiff-string model, <code>f<sub>k</sub> = k·f<sub>0</sub>·√(1 + B·k²)</code>, to all of them. Real strings are slightly stiff, so their overtones run sharp. The fit separates that from the pitch, so the reading stays exact when a phone microphone barely picks up the low E. A true strobe shows the string's phase against the target: it stands still only when the string is exactly in tune.</p>

<h2>Measured accuracy</h2>
<div class="card">
<table>
<caption class="muted">95th-percentile error in cents (1/100 of a semitone), 160 test plucks per row</caption>
<thead><tr><th scope="col">Condition</th><th scope="col">RioTuner</th><th scope="col">Classic YIN tuner</th></tr></thead>
<tbody>
<tr><td>Clean pluck</td><td>0.00</td><td>0.93</td></tr>
<tr><td>Real-string overtone scatter</td><td>0.12</td><td>0.97</td></tr>
<tr><td>Weak fundamental (phone microphone)</td><td>0.27</td><td>1.81</td></tr>
<tr><td>Hard pluck, pitch glide</td><td>0.14</td><td>1.00</td></tr>
<tr><td>Noisy room</td><td>0.12</td><td>3.95</td></tr>
<tr><td>Very noisy room</td><td>0.15</td><td>46.7</td></tr>
</tbody>
</table>
<p><small>Synthetic plucked strings with known pitch (B1 to E4, random detuning and string stiffness). Accuracy on a real phone also depends on its microphone; device tests are in progress.</small></p>
</div>

<h2>Features</h2>
<ul>
<li>16 tunings: guitar (standard, drop D, half and full step down, drop C, DADGAD, open G/D/E, 7-string), bass (4- and 5-string), ukulele, bouzouki (CFAD, DAD), baglamas, plus chromatic mode</li>
<li>Automatic string detection, or tap a string to lock it</li>
<li>Reference pitch from 380 to 480 Hz in 0.1 Hz steps</li>
<li>Note names in C♯, D♭ or Do Re Mi</li>
<li>No ads, no account, no internet permission</li>
</ul>

<h2>Privacy</h2>
<p>Sound is analysed on the phone and immediately discarded. Nothing is recorded, stored or sent. <a href="{policy}">RioTuner privacy policy</a></p>
"#,
        policy = href("riotuner/privacy/"),
    );
    Page {
        slug: "riotuner/",
        title: "RioTuner: precision guitar tuner | PelaosTech",
        description: "RioTuner measures every overtone of a string for sub-cent tuning accuracy, with a true strobe. Guitar, bass, ukulele, bouzouki. No ads, offline.",
        body,
        crumbs: vec![("riotuner/", "RioTuner")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn riotuner_privacy() -> Page {
    let body = format!(
        r#"
<h1>RioTuner privacy policy</h1>
<p class="muted">Last updated 18 October 2026</p>
<p>RioTuner is a musical instrument tuner made by PelaosTech.</p>
<h2>Microphone</h2>
<p>RioTuner uses your device's microphone only while the tuner screen is open, to measure the pitch of the instrument you are playing. The sound is analysed in memory on your device, a fraction of a second at a time, and then discarded. RioTuner does not record, save or transmit audio, and it stops listening when you leave the app.</p>
<h2>Data collection</h2>
<p>RioTuner collects no personal data. It has no accounts, advertising, analytics or crash reporting, and the Android app does not request internet access, so it cannot send anything off your device.</p>
<p>Your settings (reference pitch, tuning, display options) are stored only on your device, in the app's private storage, and are removed when you uninstall the app.</p>
<h2>Children</h2>
<p>RioTuner is suitable for all ages and collects no data from anyone.</p>
<h2>Changes</h2>
<p>If this policy changes, the new version will be published at this address with a new date.</p>
<h2>Contact</h2>
<p>Questions about this policy: <a href="{CONTACT}">open an issue on GitHub</a>.</p>
"#
    );
    Page {
        slug: "riotuner/privacy/",
        title: "RioTuner privacy policy | PelaosTech",
        description: "RioTuner analyses sound on your phone only. It records nothing, collects no data and has no internet permission.",
        body,
        crumbs: vec![("riotuner/", "RioTuner"), ("riotuner/privacy/", "Privacy policy")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn site_privacy() -> Page {
    let body = format!(
        r#"
<h1>Website privacy</h1>
<p>This website has no cookies, no analytics, no tracking and no forms. Its only script is the depth slider on the research page, which runs entirely in your browser.</p>
<p>The site is hosted on GitHub Pages. GitHub states that <q>when a GitHub Pages site is visited, the visitor's IP address is logged and stored for security purposes</q> (<a href="{GITHUB_PAGES_PRIVACY}">GitHub Docs</a>). PelaosTech does not receive or use those logs. GitHub's handling of them is covered by the GitHub General Privacy Statement.</p>
<p>The RioTuner app has its own <a href="{policy}">privacy policy</a>.</p>
"#,
        policy = href("riotuner/privacy/"),
    );
    Page {
        slug: "privacy/",
        title: "Website privacy | PelaosTech",
        description: "The PelaosTech website uses no cookies, analytics or forms. GitHub Pages logs visitor IP addresses for security.",
        body,
        crumbs: vec![("privacy/", "Website privacy")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn not_found() -> Page {
    let body = format!(
        r#"
<h1>Page not found</h1>
<p>That page does not exist. Try the <a href="{home}">home page</a>, the <a href="{research}">research page</a> or <a href="{riotuner}">RioTuner</a>.</p>
"#,
        home = href(""),
        research = href("research/"),
        riotuner = href("riotuner/"),
    );
    Page {
        slug: "404.html",
        title: "Page not found | PelaosTech",
        description: "This page does not exist on the PelaosTech website.",
        body,
        crumbs: vec![],
        extra_ld: vec![],
        script: false,
        noindex: true,
    }
}

const FAVICON: &str = r##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 108 108"><rect width="108" height="108" rx="24" fill="#070C18"/><path d="M31.9 56.4A25 25 0 0 1 76.1 56.4" fill="none" stroke="#39D0F2" stroke-width="5" stroke-linecap="round"/><path d="M54 68V38" stroke="#E9EEF7" stroke-width="4.5" stroke-linecap="round"/><circle cx="54" cy="68" r="5" fill="#E9EEF7"/></svg>
"##;

fn pages() -> Vec<Page> {
    vec![home(), research(), riotuner(), riotuner_privacy(), site_privacy(), not_found()]
}

pub fn site() -> Vec<File> {
    let pages = pages();
    let mut files: Vec<File> = pages
        .iter()
        .map(|p| File {
            path: if p.slug.ends_with(".html") { p.slug.to_string() } else { format!("{}index.html", p.slug) },
            contents: layout(p),
        })
        .collect();

    let urls = pages
        .iter()
        .filter(|p| !p.noindex)
        .map(|p| format!("  <url><loc>{}</loc><lastmod>{UPDATED}</lastmod></url>", url(p.slug)))
        .collect::<Vec<_>>()
        .join("\n");
    files.push(File {
        path: "sitemap.xml".into(),
        contents: format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n{urls}\n</urlset>\n"
        ),
    });
    // Crawlers only read robots.txt and llms.txt at a domain root; they take effect once
    // the site has its own domain. Until then, submit the sitemap URL directly.
    files.push(File { path: "robots.txt".into(), contents: format!("User-agent: *\nAllow: /\nSitemap: {SITE}/sitemap.xml\n") });
    files.push(File {
        path: "llms.txt".into(),
        contents: format!(
            "# PelaosTech\n\n> Marine robotics research (deep-submergence ROV concept) and precision software (RioTuner, an instrument tuner).\n\n- [Research: pressure and navigation]({}): pressure at depth with NOAA sources, ROV vs AUV, navigation without GPS\n- [RioTuner]({}): how the tuner measures pitch, measured accuracy, features\n- [RioTuner privacy policy]({})\n",
            url("research/"),
            url("riotuner/"),
            url("riotuner/privacy/")
        ),
    });
    files.push(File { path: "favicon.svg".into(), contents: FAVICON.to_string() });
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html() -> Vec<(String, String)> {
        site().into_iter().filter(|f| f.path.ends_with(".html")).map(|f| (f.path, f.contents)).collect()
    }

    #[test]
    fn one_h1_per_page() {
        for (path, c) in html() {
            assert_eq!(c.matches("<h1").count(), 1, "{path}");
        }
    }

    #[test]
    fn titles_and_descriptions_fit_search_results() {
        for p in pages() {
            assert!(p.title.chars().count() <= 60, "title too long: {}", p.title);
            assert!(p.description.chars().count() <= 160, "description too long: {}", p.description);
        }
    }

    #[test]
    fn no_uav_content() {
        for f in site() {
            assert!(!f.contents.to_lowercase().contains("uav"), "{}", f.path);
        }
    }

    /// GitHub Pages: no mixed content, and no forms (no sensitive transactions).
    #[test]
    fn pages_compliance() {
        for (path, c) in html() {
            assert!(!c.contains("src=\"http:") && !c.contains("href=\"http:"), "insecure URL in {path}");
            assert!(!c.contains("<form"), "form in {path}");
            assert!(!c.contains("<input"), "input in {path}");
            assert!(c.contains("form-action 'none'"), "CSP missing in {path}");
        }
    }

    #[test]
    fn internal_links_resolve() {
        let files: Vec<String> = site().into_iter().map(|f| f.path).collect();
        for (path, c) in html() {
            for part in c.split("href=\"").skip(1) {
                let link = &part[..part.find('"').unwrap()];
                if let Some(rest) = link.strip_prefix(&format!("{BASE}/")) {
                    let target = if rest.is_empty() || rest.ends_with('/') { format!("{rest}index.html") } else { rest.to_string() };
                    assert!(files.contains(&target), "{path} links to missing {link}");
                } else {
                    assert!(link.starts_with("https://") || link.starts_with('#'), "{path}: unexpected link {link}");
                }
            }
        }
    }

    #[test]
    fn canonical_and_sitemap() {
        for (path, c) in html() {
            assert!(c.contains("<link rel=\"canonical\" href=\"https://"), "{path}");
        }
        let sitemap = site().into_iter().find(|f| f.path == "sitemap.xml").unwrap().contents;
        assert_eq!(sitemap.matches("<url>").count(), 5);
        assert!(!sitemap.contains("404"));
    }

    #[test]
    fn pressure_figures() {
        assert!((gauge_bar(4000.0) - 402.1).abs() < 0.1);
        assert!((window_tonnes(4000.0) - 41.0).abs() < 0.01);
        assert_eq!(group(11000), "11,000");
    }
}
