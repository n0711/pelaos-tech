//! Page content, layout, structured data, sitemap. Tests at the bottom enforce the
//! GitHub Pages and SEO rules the site relies on.

/// Canonical origin + base path, without a trailing slash. Change these two (and add a
/// CNAME file) when the site moves to its own domain.
pub const SITE: &str = "https://n0711.github.io/pelaos-tech";
pub const BASE: &str = "/pelaos-tech";
/// Shown in the footer, JSON-LD `dateModified` and sitemap `lastmod`.
pub const UPDATED: &str = "2026-10-19";

/// Seawater density and standard gravity used for every pressure figure on the site.
pub const RHO: f64 = 1025.0;
pub const G: f64 = 9.80665;

const NOAA_PRESSURE: &str = "https://oceanservice.noaa.gov/facts/pressure.html";
const NOAA_AUV_ROV: &str = "https://oceanservice.noaa.gov/facts/auv-rov.html";
const NOAA_NAVIGATION: &str = "https://oceanexplorer.noaa.gov/expedition-feature/technology-development-partnerships-ex1904-logs-july29/";
const GITHUB_PAGES_PRIVACY: &str = "https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages";
const CONTACT: &str = "https://github.com/n0711/pelaos-tech/issues";
const LINKEDIN: &str = "https://www.linkedin.com/in/nadiotis/";

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
            c if c <= '\u{001f}' => o.push_str(&format!("\\u{:04x}", c as u32)),
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
:focus-visible{outline:3px solid var(--accent);outline-offset:4px}
header,main,footer{max-width:1080px;padding-left:24px;padding-right:24px}
main{padding-top:32px}main>p,main>ul,main>dl{max-width:780px}
header{border-bottom:1px solid var(--line);padding-top:20px;padding-bottom:20px}
h1{font-size:clamp(36px,6.5vw,72px);letter-spacing:-.055em;line-height:1.04;max-width:850px}
h2{letter-spacing:-.025em}.lead{max-width:680px!important;font-size:21px;line-height:1.65}
.eyebrow{text-transform:uppercase;font-size:12px;font-weight:750;letter-spacing:.15em;color:var(--accent)}
.hero{padding:28px 0 44px}.hero h1{margin:18px 0 24px}.hero .lead{margin-bottom:28px}
.actions{display:flex;gap:12px;flex-wrap:wrap;align-items:center;margin:24px 0}
.button{display:inline-flex;min-height:46px;align-items:center;justify-content:center;padding:11px 18px;border:1px solid var(--accent);border-radius:8px;font-weight:650;text-decoration:none}
.button.primary{background:var(--accent);color:var(--bg)}.button:hover{text-decoration:underline;text-underline-offset:4px}
.project-grid{display:grid;grid-template-columns:repeat(3,minmax(0,1fr));gap:18px}
.project{padding:24px;margin:0;display:flex;flex-direction:column;align-items:flex-start;gap:8px}
.project h3{font-size:25px;line-height:1.2;margin:16px 0 4px}.project p{margin:0 0 14px}
.project a{margin-top:auto;min-height:44px;display:inline-flex;align-items:center;font-weight:650}
.badge{display:inline-flex;border:1px solid var(--line);border-radius:30px;padding:4px 10px;font-size:12px;font-weight:650;color:var(--muted)}
.panel{display:grid;grid-template-columns:1fr 1fr;gap:32px;border-top:1px solid var(--line);padding:32px 0;margin-top:48px}
.panel h2{margin-top:0}.table-scroll{overflow-x:auto}.status-note{border-left:3px solid var(--accent);padding:8px 18px;margin:24px 0;color:var(--muted)}
@media(max-width:720px){header,main,footer{padding-left:18px;padding-right:18px}.project-grid,.panel{grid-template-columns:1fr}.hero{padding:8px 0 28px}.lead{font-size:18px}.project{padding:22px}nav a{padding:0 8px}table{font-size:14px}}

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
<p>© 2026 PelaosTech. Last updated <time datetime="{UPDATED}">19 October 2026</time>. Hosted on GitHub Pages.</p>
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
        r##"
<section class="hero">
<p class="eyebrow">PELAOS / Independent software &amp; research / Cyprus</p>
<h1>Useful software.<br>Built with a purpose.</h1>
<p class="lead">Projects by Charalampos Nadiotis, connecting music, learning and local culture with practical engineering.</p>
<div class="actions"><a class="button primary" href="#projects">Explore the projects ↓</a><a class="button" href="{LINKEDIN}">Meet the maker ↗</a></div>
</section>
<section id="projects" aria-labelledby="project-heading">
<p class="eyebrow">Selected projects</p><h2 id="project-heading">Different interests. Things you can use.</h2>
<div class="project-grid">
<article class="card project"><span class="badge">Android · Pre-release</span><h3>RioTuner</h3><p>An offline instrument tuner with needle and strobe displays. Built for guitar, bass, ukulele and bouzouki. Physical-device validation is still required.</p><a href="{riotuner}">Explore RioTuner →</a></article>
<article class="card project"><span class="badge">Live · Community website</span><h3>cysk8clips</h3><p>Cyprus skateboarding clips, full parts, event stories and a public spot guide. A record of the people and places behind the scene.</p><a href="https://cysk8clips.com">Visit the skate archive ↗</a></article>
<article class="card project"><span class="badge">Open source · Local planner</span><h3>LearnTwin</h3><p>Track degree progress, study tasks, thesis milestones and PhD applications. Records stay in your browser; export a backup to keep them.</p><a href="https://github.com/n0711/LearnTwin#start-the-planner">Try LearnTwin from source ↗</a></article>
</div>
</section>
<section class="panel"><div><p class="eyebrow">Research notebook</p><h2>Curiosity goes deeper.</h2></div><div><p>Marine robotics is a research interest: pressure, navigation and the challenges of operating underwater. The deep-submergence vehicle is a concept, not a deployed product.</p><a href="{research}">Explore the deep-sea notes →</a></div></section>
<section class="panel"><div><p class="eyebrow">The person behind PELAOS</p><h2>From Cyprus, with curiosity.</h2></div><div><p>I’m Charalampos Nadiotis, a robotics engineer and Computer Science student. This is my project portfolio: what is available, what is being tested and what I’m exploring next.</p><p><a href="https://github.com/n0711">GitHub</a> · <a href="{LINKEDIN}">Contact on LinkedIn</a></p></div></section>
"##,
        research = href("research/"),
        riotuner = href("riotuner/"),
    );
    Page {
        slug: "",
        title: "PELAOS | Software, music & community projects",
        description: "Explore PELAOS projects: RioTuner, cysk8clips and LearnTwin. Independent software and marine robotics research by Charalampos Nadiotis in Cyprus.",
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
<p class="lead">A precision tuner for guitar, bass, ukulele and bouzouki, for Android. Pre-release software; a Google Play release is planned. A verified store link is not available yet.</p>

<h2>How it measures</h2>
<p>RioTuner estimates pitch from multiple string partials and fits a stiff-string model, <code>f<sub>k</sub> = k·f<sub>0</sub>·√(1 + B·k²)</code>, to all of them. Real strings are slightly stiff, so their overtones run sharp. The fit aims to separate string stiffness from pitch, including when the fundamental is weak. The strobe visualises phase drift relative to the target pitch. Real-device accuracy has not yet been established.</p>

<p class="status-note">No purchase or download is offered here yet. Review the testing guide to see what must be verified before release.</p>
<p><a class="button primary" href="{testing}">Read the testing guide →</a></p>
<h2>Synthetic benchmark results</h2>
<div class="card">
<table>
<caption class="muted">95th-percentile error on synthetic signals, in cents; 160 trials per row</caption>
<thead><tr><th scope="col">Condition</th><th scope="col">RioTuner</th><th scope="col">Textbook YIN baseline</th></tr></thead>
<tbody>
<tr><td>Clean pluck</td><td>0.00</td><td>0.93</td></tr>
<tr><td>Real-string overtone scatter</td><td>0.12</td><td>0.97</td></tr>
<tr><td>Simulated weak fundamental</td><td>0.27</td><td>1.81</td></tr>
<tr><td>Hard pluck, pitch glide</td><td>0.14</td><td>1.00</td></tr>
<tr><td>Noisy room</td><td>0.12</td><td>3.95</td></tr>
<tr><td>Very noisy room</td><td>0.15</td><td>46.7</td></tr>
</tbody>
</table>
<p><small>Synthetic plucked strings with known pitch (B1 to E4, random detuning and string stiffness). This is an algorithm benchmark, not a comparison with commercial tuner apps or a measured result on physical phones. Device accuracy and latency remain unverified.</small></p>
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
        testing = href("riotuner/testing/"),
    );
    Page {
        slug: "riotuner/",
        title: "RioTuner: precision guitar tuner | PelaosTech",
        description: "RioTuner: an offline Android instrument tuner with strobe and needle displays. Pre-release; synthetic benchmarks and physical-device testing requirements.",
        body,
        crumbs: vec![("riotuner/", "RioTuner")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn riotuner_testing() -> Page {
    Page {
        slug: "riotuner/testing/",
        title: "RioTuner testing guide | PELAOS",
        description: "How to evaluate a RioTuner test build: microphone permission, tuning response, interruptions and device details. Physical-device validation is still pending.",
        body: format!(r#"
<p class="eyebrow">RioTuner / Before release</p>
<h1>Put it through a real practice session.</h1>
<p class="lead">Synthetic signals test the algorithm. Instruments and phones test the product.</p>
<p class="status-note">No public test build or Play opt-in link has been verified. This guide prepares testing; it does not enrol you in a test or provide a download.</p>
<h2>When a build is available</h2>
<ol><li>Check that the build comes from the developer or the official Play testing track.</li><li>Allow microphone access, then deny it once and check that the app explains how to continue.</li><li>Play each open string separately in a quiet room. Check string selection, the displayed note and response to a slow tuning change.</li><li>Compare needle and strobe displays, then repeat with background noise and a weaker low note.</li><li>Switch apps, lock and unlock the phone, and reopen the tuner. Microphone use should stop in the background and recover correctly.</li><li>Change reference pitch and tuning, restart the app and check that your settings are retained.</li></ol>
<h2>Useful feedback</h2>
<p>Include app version, phone model, Android version, instrument, tuning, expected behaviour and what actually happened. Report crashes and permission problems as well as tuning behaviour.</p>
<p>A comparison with another tuner is a usability check, not calibrated accuracy evidence. Quantitative accuracy claims need a documented reference and measurement setup.</p>
<p><a class="button" href="{LINKEDIN}">Ask about testing on LinkedIn ↗</a> <a href="{CONTACT}">Report a non-sensitive issue</a></p>
<p class="muted">GitHub issues are public. Do not post contact details, private recordings, account information or device identifiers.</p>
"#),
        crumbs: vec![("riotuner/", "RioTuner"), ("riotuner/testing/", "Testing")],
        extra_ld: vec![], script: false, noindex: false,
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
    vec![home(), research(), riotuner(), riotuner_testing(), riotuner_privacy(), site_privacy(), not_found()]
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
            "# PelaosTech\n\n> Marine robotics research (deep-submergence ROV concept) and precision software (RioTuner, an instrument tuner).\n\n- [Research: pressure and navigation]({}): pressure at depth with NOAA sources, ROV vs AUV, navigation without GPS\n- [RioTuner]({}): pre-release pitch detection, synthetic benchmark results and limits; microphone and device accuracy remain unverified\n- [RioTuner privacy policy]({})\n",
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
    fn json_escapes_every_control_character() {
        for code in 0u8..=31 {
            let input = char::from(code).to_string();
            let expected = if code == b'\n' {
                "\"\\n\"".to_string()
            } else {
                format!("\"\\u{code:04x}\"")
            };
            assert_eq!(js(&input), expected, "control character {code}");
        }
    }

    #[test]
    fn json_preserves_unicode_and_escapes_script_delimiters() {
        assert_eq!(js("α海"), "\"α海\"");
        assert_eq!(js("\"\\\r\t\n</script>"), "\"\\\"\\\\\\u000d\\u0009\\n\\u003c/script>\"");
    }

    #[test]
    fn crawler_summary_qualifies_tuner_benchmarks() {
        let summary = site().into_iter().find(|f| f.path == "llms.txt").unwrap().contents;
        assert!(summary.contains("synthetic benchmark results and limits"));
        assert!(summary.contains("microphone and device accuracy remain unverified"));
        assert!(!summary.contains("measured accuracy"));
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
        assert_eq!(sitemap.matches("<url>").count(), 6);
        assert!(!sitemap.contains("404"));
    }

    #[test]
    fn pressure_figures() {
        assert!((gauge_bar(4000.0) - 402.1).abs() < 0.1);
        assert!((window_tonnes(4000.0) - 41.0).abs() < 0.01);
        assert_eq!(group(11000), "11,000");
    }
}
