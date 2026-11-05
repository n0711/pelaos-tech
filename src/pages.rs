//! Page content, layout, structured data, sitemap. Tests at the bottom enforce the
//! Static-hosting and SEO rules the site relies on.

/// Compile-time deployment settings. Unset values preserve GitHub Pages builds.
pub const SITE: &str = match option_env!("PELAOS_SITE_URL") {
    Some(value) => value,
    None => "https://n0711.github.io/pelaos-tech",
};
pub const BASE: &str = match option_env!("PELAOS_BASE_PATH") {
    Some(value) => value,
    None => "/pelaos-tech",
};
const HOSTING: &str = match option_env!("PELAOS_HOSTING") {
    Some(value) => value,
    None => "github-pages",
};
/// Shown in the footer, JSON-LD `dateModified` and sitemap `lastmod`.
pub const UPDATED: &str = "2026-11-05";

pub fn validate_config() -> std::io::Result<()> {
    let invalid = || std::io::Error::new(std::io::ErrorKind::InvalidInput, "deployment needs an HTTPS site URL without a trailing slash, a matching base path, and github-pages or vercel hosting");
    let Some(site) = SITE.strip_prefix("https://") else { return Err(invalid()); };
    if site.is_empty()
        || SITE.ends_with('/')
        || SITE.chars().any(|c| c.is_whitespace() || matches!(c, '"' | '\'' | '<' | '>' | '?' | '#' | '\\'))
        || (!BASE.is_empty() && (!BASE.starts_with('/') || BASE.ends_with('/') || BASE.contains("//") || BASE.split('/').any(|part| part == "." || part == "..")))
        || !matches!(HOSTING, "github-pages" | "vercel")
    {
        return Err(invalid());
    }
    let Some(origin) = site.strip_suffix(BASE) else { return Err(invalid()); };
    if origin.is_empty() || origin.contains('/') { return Err(invalid()); }
    Ok(())
}

fn hosting_name() -> &'static str {
    if HOSTING == "vercel" { "Vercel" } else { "GitHub Pages" }
}

/// Seawater density and standard gravity used for every pressure figure on the site.
pub const RHO: f64 = 1025.0;
pub const G: f64 = 9.80665;
/// Range of the research-page calculator, not a vehicle or sensor depth rating.
pub const MAX_DEPTH_M: u64 = 500;
const INITIAL_DEPTH_M: u64 = 100;

const NOAA_PRESSURE: &str = "https://oceanservice.noaa.gov/facts/pressure.html";
const NOAA_AUV_ROV: &str = "https://oceanservice.noaa.gov/facts/auv-rov.html";
const NOAA_NAVIGATION: &str = "https://oceanexplorer.noaa.gov/expedition-feature/technology-development-partnerships-ex1904-logs-july29/";
const GITHUB_PAGES_PRIVACY: &str = "https://docs.github.com/en/pages/getting-started-with-github-pages/what-is-github-pages";
const CONTACT: &str = "https://github.com/n0711/pelaos-tech/issues";
const LINKEDIN: &str = "https://www.linkedin.com/in/nadiotis/";
const RESEARCHGATE: &str = "https://www.researchgate.net/profile/Charalampos-Nadiotis-2";
/// Voluntary support link, also used by `.github/FUNDING.yml` in each repository.
const REVOLUT: &str = "https://revolut.me/nadiotis";

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

/// Use the same half-up rounding as the browser, including 5.125 tonnes at 500 m.
fn fixed2(value: f64) -> String {
    format!("{:.2}", (value * 100.0).round() / 100.0)
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
.project-grid{display:grid;grid-template-columns:repeat(2,minmax(0,1fr));gap:18px}
.project{padding:24px;margin:0;display:flex;flex-direction:column;align-items:flex-start;gap:8px}
.project h3{font-size:25px;line-height:1.2;margin:16px 0 4px}.project p{margin:0 0 14px}
.project.wide{grid-column:1/-1}.project a{margin-top:auto;min-height:44px;display:inline-flex;align-items:center;font-weight:650}
.badge{display:inline-flex;border:1px solid var(--line);border-radius:30px;padding:4px 10px;font-size:12px;font-weight:650;color:var(--muted)}
.panel{display:grid;grid-template-columns:1fr 1fr;gap:32px;border-top:1px solid var(--line);padding:32px 0;margin-top:48px}
.panel h2{margin-top:0}.table-scroll{overflow-x:auto}.status-note{border-left:3px solid var(--accent);padding:8px 18px;margin:24px 0;color:var(--muted)}
.evidence-table th,.evidence-table td{text-align:left;vertical-align:top}.evidence-table th:nth-child(2),.evidence-table td:nth-child(2){white-space:nowrap}
html{scroll-padding-top:12px}
body{overflow-wrap:break-word}
main,.project,.panel>*{min-width:0}
nav{display:flex;align-items:center;flex-wrap:wrap}
nav a{white-space:nowrap}
@media(max-width:720px){
header,main,footer{padding-left:18px;padding-right:18px}
header{display:block;padding-top:14px;padding-bottom:8px}
.brand{display:inline-block;margin-bottom:8px}
nav{display:flex;flex-wrap:nowrap;gap:4px;max-width:100%;overflow-x:auto;overscroll-behavior-x:contain;scrollbar-width:thin;-webkit-overflow-scrolling:touch;padding-bottom:5px}
nav a{flex:0 0 auto;padding:0 12px}
.project-grid,.panel{grid-template-columns:1fr}
.hero{padding:8px 0 28px}.lead{font-size:18px}.project{padding:22px}
table{font-size:14px}
h1{overflow-wrap:anywhere}
}

"#;

fn layout(p: &Page) -> String {
    let canonical = url(p.slug);
    let nav = [("", "Home"), ("services/", "Websites"), ("research/", "Research"), ("vera-mot/", "VERA-MOT"), ("ros3-limassol/", "ROS3"), ("riotuner/", "RioTuner")]
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
<link rel="manifest" href="{BASE}/manifest.webmanifest">
<link rel="apple-touch-icon" href="{BASE}/icon-192.png">
<meta name="theme-color" content="#070c18">
<meta name="apple-mobile-web-app-title" content="PelaosTech">
<meta name="color-scheme" content="light dark">
<meta property="og:type" content="website">
<meta property="og:locale" content="en_GB">
<meta property="og:site_name" content="PelaosTech">
<meta property="og:title" content="{title}">
<meta property="og:description" content="{desc}">
<meta property="og:url" content="{canonical}">
<meta name="twitter:card" content="summary">
<meta name="twitter:title" content="{title}">
<meta name="twitter:description" content="{desc}">
<style>{css}</style>
<script type="application/ld+json">{ld}</script>
{script}<script src="{BASE}/pwa.js" defer></script>
</head>
<body>
<a class="skip" href="#main">Skip to content</a>
<header><a class="brand" href="{home}">PelaosTech</a><nav aria-label="Main">{nav}</nav></header>
<main id="main">
{body}
</main>
<footer>
<a href="{services}">Websites</a><a href="{research}">Research</a><a href="{riotuner}">RioTuner</a><a href="{privacy}">Website privacy</a><a href="{CONTACT}">Contact</a><a href="{REVOLUT}">Support ↗</a>
<p>© 2026 PelaosTech. Last updated <time datetime="{UPDATED}">5 November 2026</time>. Hosted on {hosting}.</p>
</footer>
</body>
</html>
"##,
        title = esc(p.title),
        desc = esc(p.description),
        css = CSS.trim(),
        home = href(""),
        services = href("services/"),
        research = href("research/"),
        riotuner = href("riotuner/"),
        privacy = href("privacy/"),
        hosting = hosting_name(),
        body = p.body.trim(),
    )
}

fn home() -> Page {
    let body = format!(
        r##"
<section class="hero">
<p class="eyebrow">PELAOS / Independent software &amp; research / Cyprus</p>
<h1>Independent software, AI and open research from Cyprus.</h1>
<p class="lead">Explore original software, computer vision, distributed systems and mathematical research, alongside practical applications and an early website pilot.</p>
<div class="actions"><a class="button primary" href="#projects">Explore the projects ↓</a><a class="button" href="{services}">Website pilot →</a></div>
</section>
<section id="projects" aria-labelledby="project-heading">
<p class="eyebrow">Selected projects</p><h2 id="project-heading">Different interests. Things you can use.</h2>
<div class="project-grid">
<article class="card project"><span class="badge">Android · Coming to Google Play</span><h3>RioTuner</h3><p>An offline instrument tuner with needle and strobe displays for guitar, bass, ukulele and bouzouki. Development is finished and the Google Play release is being prepared.</p><a href="{riotuner}">Explore RioTuner →</a></article>
<article class="card project"><span class="badge">Live · Community website</span><h3>cysk8clips</h3><p>Cyprus skateboarding clips, full parts, event stories and a public spot guide. A record of the people and places behind the scene.</p><a href="https://cysk8clips.com">Visit the skate archive ↗</a></article>
<article class="card project"><span class="badge">OpenPELAOS · Team prototype</span><h3>Cultural Storytelling Map</h3><p>Narrify connects community stories with places. Co-developed by Charalampos Nadiotis, Hussain Jamil and Nikitas Savva.</p><a href="{narrify}">Explore Narrify →</a></article>
<article class="card project"><span class="badge">Open source · Local planner</span><h3>LearnTwin</h3><p>Track degree progress, study tasks, thesis milestones and PhD applications. Records stay in your browser; export a backup to keep them.</p><a href="https://github.com/n0711/LearnTwin#start-the-planner">Try LearnTwin from source ↗</a></article>
<article class="card project wide"><span class="badge">OpenPELAOS · ROS 2 Humble / Jazzy</span><h3>ROS3-Limassol</h3><p>Telemetry arrival-liveness monitoring for robots with intermittent connectivity. Evidence now includes synthetic replay, ArduPilot Rover SITL over localhost and a limited Raspberry Pi–Arduino USB disconnection bench test. Flight-controller, radio-link, vessel/robot and field validation remain pending. Original idea: Christos Paparistodemou. Project lead and manuscript author: Charalampos Nadiotis. This independent prototype is not an official ROS release.</p><a href="{ros3}">Explore ROS3 research →</a></article>
<article class="card project wide"><span class="badge">Open research · Synthetic verification</span><h3>VERA-MOT</h3><p>Quality-gated appearance association for multi-object tracking in drone video. An opt-in BoT-SORT integration now protects stored identity features against low-quality crops in synthetic tests. The implementation note is available in XeLaTeX and PDF; real vehicle identity evaluation remains unverified.</p><a href="{vera}">Read the VERA-MOT research note →</a></article>
</div>
</section>
<section class="panel"><div><p class="eyebrow">Website pilot / Cyprus</p><h2>A clear website for a small business.</h2></div><div><p>PELAOS is preparing a fixed-scope, one-page website pilot with responsive design, contact links, basic search metadata and source handover. It is a proposal rather than an open booking service.</p><a href="{services}">Read the website pilot scope →</a></div></section>
<section class="panel"><div><p class="eyebrow">Research notebook</p><h2>Algorithms, AI and experimental systems.</h2></div><div><p>Follow ongoing work on P versus NP (an open question, not a claimed proof), reliability-gated vehicle tracking and ROS3 telemetry monitoring. Additional marine engineering notes cover underwater pressure and navigation; experimental limits are stated for each project.</p><a href="{research}">Browse all research notes →</a></div></section>
<section class="panel"><div><p class="eyebrow">The person behind PELAOS</p><h2>From Cyprus, with curiosity.</h2></div><div><p>I’m Charalampos Nadiotis, a robotics engineer and Computer Science student at Frederick University. This is my project portfolio: what is available, what is being tested and what I’m exploring next.</p><p><a href="https://github.com/n0711">GitHub</a> · <a href="{RESEARCHGATE}">ResearchGate</a> · <a href="{LINKEDIN}">Contact on LinkedIn</a></p></div></section>
<section class="panel" id="support"><div><p class="eyebrow">Support</p><h2>Keep the projects going.</h2></div><div><p>The projects here are free and built in my own time. If one of them is useful to you, you can support further development with a one-off contribution via Revolut. Every contribution is voluntary and does not buy access, features or support.</p><a class="button primary" href="{REVOLUT}">Support via Revolut ↗</a></div></section>
"##,
        research = href("research/"),
        riotuner = href("riotuner/"),
        services = href("services/"),
        narrify = href("cultural-storytelling-map/"),
        ros3 = href("ros3-limassol/"),
        vera = href("vera-mot/"),
    );
    Page {
        slug: "",
        title: "PELAOS Tech | Software, AI and Computer Science Research",
        description: "Independent software, AI and computer science research from PELAOS: VERA-MOT, ROS3-Limassol, RioTuner, Narrify, LearnTwin and more.",
        body,
        crumbs: vec![],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}




fn services() -> Page {
    Page {
        slug: "services/",
        title: "Small Business Websites in Cyprus | PELAOS",
        description: "PELAOS is preparing a one-page website pilot for small businesses in Cyprus, with responsive design, basic SEO, contact links and source handover.",
        body: format!(r##"
<section class="hero">
<p class="eyebrow">PELAOS / Cyprus / Proposed website pilot</p>
<h1>Small-business websites in Cyprus</h1>
<p class="lead">A proposed fixed-scope website service for local businesses that need one clear page explaining what they do and how to get in touch.</p>
<div class="actions"><a class="button primary" href="{LINKEDIN}">Ask about pilot availability ↗</a><a class="button" href="{home}">See PELAOS projects →</a></div>
</section>
<section aria-labelledby="scope-title">
<h2 id="scope-title">What the one-page pilot would include</h2>
<ul>
<li>Responsive layout for phone and desktop.</li>
<li>Sections for services, contact links and a location or service area supplied by the business.</li>
<li>A descriptive HTML title, meta description and social-sharing metadata.</li>
<li>One consolidated review round, source handover and basic editing instructions.</li>
</ul>
<p>Search-friendly content and metadata help search engines understand a page, but do not guarantee rankings, traffic or enquiries.</p>
</section>
<section aria-labelledby="inputs-title">
<h2 id="inputs-title">What we would need from a client</h2>
<p>Final approved business text, permission to use any images and brand assets, preferred contact details and the agreed hosting arrangement. Copywriting, translation, domains, hosting subscriptions, booking systems and ongoing maintenance would require separate agreement.</p>
</section>
<section aria-labelledby="status-title">
<h2 id="status-title">Pilot status and next step</h2>
<p>This is an early service proposal, not a live ordering or payment page. Scope, schedule, availability, price, taxes and cancellation terms must be confirmed in a written quote before any work or payment. No lead form, tracking or checkout is used here.</p>
<p>To discuss a potential pilot, <a href="{LINKEDIN}">contact PELAOS on LinkedIn</a>. For a sample of current engineering and software work, explore the <a href="{research}">computer science and AI research notes</a>.</p>
</section>
"##, home = href(""), research = href("research/")),
        crumbs: vec![("services/", "Website services")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn vera_mot() -> Page {
    Page {
        slug: "vera-mot/",
        title: "VERA-MOT ReID Research | PELAOS",
        description: "VERA-MOT research on quality-gated vehicle appearance matching and protected feature memory: open source, synthetic tests and validation limits.",
        body: r##"
<section class="hero">
<p class="eyebrow">PELAOS / Computer vision research / 3 November 2026</p>
<h1>VERA-MOT: reliability-gated vehicle tracking</h1>
<p class="lead">Can low-quality appearance observations be prevented from corrupting the identity memory of a multi-object tracker?</p>
<p class="status-note"><strong>Preliminary implementation study:</strong> The current results establish synthetic code and pinned-tracker behaviour, not improved vehicle tracking accuracy. The manuscript is a working technical note, not a peer-reviewed paper or a validated vehicle benchmark.</p>
<div class="actions">
<a class="button primary" href="https://github.com/n0711/vera-mot/blob/main/docs/papers/VERA_MOT_Implementation_Note.pdf">Read the XeLaTeX-built paper (PDF) ↗</a>
<a class="button" href="https://github.com/n0711/vera-mot/blob/main/docs/papers/VERA_MOT_Implementation_Note.tex">View editable XeLaTeX source ↗</a>
<a class="button" href="https://github.com/n0711/vera-mot">Explore source repository ↗</a>
</div>
</section>
<section aria-labelledby="vera-method-title">
<p class="eyebrow">Mechanism / M4.6 research checkpoint</p>
<h2 id="vera-method-title">How reliability-gated ReID works</h2>
<ol>
<li>Receive a video frame and detections with bounding-box coordinates and scores.</li>
<li>Compute an appearance descriptor for each selected crop, alongside a provisional quality score from crop area, sharpness and in-frame extent.</li>
<li>When the image-quality score is low, exclude that descriptor from appearance association and persistent identity-feature updates.</li>
<li>Keep the tracker's geometric and motion association route available; accepted observations may update appearance memory again.</li>
</ol>
<p>The implementation uses an opt-in patch to a pinned BoT-SORT revision. The quality measure is an uncalibrated heuristic, not a probability of correct identity; in-frame extent does not detect occlusion behind another object. Appearance descriptors are still computed before gating, so reduced inference time is not claimed.</p>
</section>
<section aria-labelledby="vera-evidence-title">
<p class="eyebrow">Evidence / Verified 3 November 2026</p>
<h2 id="vera-evidence-title">What has actually been tested?</h2>
<div class="table-scroll"><table class="evidence-table">
<caption class="muted">Synthetic and code-level tests only; no new real-vehicle evaluation</caption>
<thead><tr><th scope="col">Check</th><th scope="col">Verified scope</th></tr></thead>
<tbody>
<tr><td>CPU unit tests</td><td>121 passed; five integration tests deselected in the recorded GitHub CI job</td></tr>
<tr><td>Pinned BoT-SORT integration</td><td>Four passed synthetic integration tests; protected appearance memory and missing-feature handling checked</td></tr>
<tr><td>Opt-in compatibility patches</td><td>Patch applicability and repeated local installer behaviour verified in CI</td></tr>
<tr><td>Vehicle identity benchmark</td><td>Not established. No new IDF1, HOTA, association accuracy, identity-switch or runtime improvement is claimed</td></tr>
</tbody></table></div>
<p>The earlier no-ReID tracking metrics are archival repository summaries; they were not regenerated here and cannot be compared to a missing real-vehicle VERA run. <a href="https://github.com/n0711/vera-mot/blob/main/docs/checkpoints/M4_6_REID_2026-11-03.md">Read the dated checkpoint and evidence limits ↗</a>.</p>
</section>
<section class="panel"><div><p class="eyebrow">Publication status</p><h2>Working technical note, not peer reviewed</h2></div><div>
<p><strong>Author:</strong> Charalampos Nadiotis. The five-page paper describes the mathematical gate, source integration, testing constraints, reproducibility, threats to validity and a prospective evaluation protocol.</p>
<p>The XeLaTeX source and PDF are hosted on GitHub. The note has not been uploaded to ResearchGate as an empirical preprint.</p>
<p><a href="https://github.com/n0711/vera-mot/blob/main/docs/VERA_REID_RELIABILITY.md">Implementation and test protocol ↗</a></p>
</div></section>
<section class="panel"><div><p class="eyebrow">Next experiment</p><h2>What would validate the hypothesis?</h2></div><div>
<p>Use identity-annotated vehicle sequences, fixed detections and identical tracker settings to compare no ReID, ungated appearance matching and VERA-gated appearance matching. Record identity switches, IDF1, AssA, HOTA, false rejection and latency by sequence with full input provenance.</p>
<p>Until then, the implementation is open and testable but the proposed tracking benefit remains an open empirical question.</p>
</div></section>
"##.to_string(),
        crumbs: vec![("vera-mot/", "VERA-MOT")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn ros3_limassol() -> Page {
    Page {
        slug: "ros3-limassol/",
        title: "ROS3-Limassol | OpenPELAOS Robotics Research",
        description: "ROS3-Limassol evidence: synthetic replay, Rover SITL loopback and a limited USB bench test, with radio-link and field validation gaps.",
        body: r##"
<section class="hero">
<p class="eyebrow">OpenPELAOS / Robotics research / Experimental</p>
<h1>ROS3-Limassol</h1>
<p class="lead">A reproducible telemetry arrival-liveness monitor for robotic systems with intermittent connections. The question is simple: how can an observer detect that accepted telemetry stopped arriving, then record its recovery?</p>
<div class="actions"><a class="button primary" href="https://github.com/n0711/ROS3-Limassol">Explore the source ↗</a><a class="button" href="https://github.com/n0711/ROS3-Limassol/blob/main/robotics/ros3_limassol/docs/VALIDATION.md">Read validation notes ↗</a></div>
</section>
<section aria-labelledby="ros3-flow-title">
<p class="eyebrow">Receiver-level observation</p>
<h2 id="ros3-flow-title">A reproducible connection-interruption experiment</h2>
<ol>
<li><strong>WAITING:</strong> No accepted telemetry has arrived in this receiver session.</li>
<li><strong>HEALTHY:</strong> A valid telemetry message has arrived within the configured receiver-time interval.</li>
<li><strong>STALE:</strong> The interval expires with no newly accepted arrival, including when the network is silent.</li>
<li><strong>RECOVERY:</strong> New accepted arrivals return the monitor to HEALTHY. The recording can be replayed offline.</li>
</ol>
<p class="status-note"><strong>Important:</strong> HEALTHY describes recent receiver arrivals only. It does not prove sensor data freshness, safe navigation, robot health or reliable connectivity in the field.</p>
</section>
<section class="panel"><div><p class="eyebrow">Implementation</p><h2>What the code does</h2></div><div>
<ul><li>Strict versioned JSON telemetry with bounded inputs and duplicate/out-of-order rejection.</li>
<li>Deterministic synthetic scenarios for communication blackout and recovery.</li>
<li>Streaming JSONL recordings and offline replay with different timeout settings.</li>
<li>Optional ROS 2 Humble and Jazzy adapters, plus an experimental Rust validator and replay core.</li></ul>
<p>No vehicle-command interface is included. ROS3-Limassol is an independent research prototype built around ROS 2, not an official ROS distribution or successor.</p>
</div></section>
<section class="panel"><div><p class="eyebrow">Research attribution</p><h2>Credits and manuscript</h2></div><div>
<p><strong>Original project idea:</strong> <a href="https://www.researchgate.net/profile/Christos-Paparistodemou">Christos Paparistodemou</a>.</p>
<p><strong>Project lead, implementation and paper author:</strong> <a href="https://github.com/n0711">Charalampos Nadiotis</a>.</p>
<p><a href="https://github.com/n0711/ROS3-Limassol/blob/main/docs/papers/ROS3_Limassol_ResearchGate_No_Military_Use.pdf">Read the preliminary technical report (PDF) ↗</a></p>
<p>The October 2026 manuscript is preliminary and not peer-reviewed. Its <a href="https://github.com/n0711/ROS3-Limassol/blob/main/docs/papers/MANUSCRIPT_LICENCE.md">Civilian-Use Licence</a> prohibits military use of that manuscript and adaptations; the separately licensed software has different terms.</p>
</div></section>
<section aria-labelledby="ros3-status-title">
<p class="eyebrow">Evidence and limits</p>
<h2 id="ros3-status-title">Current validation status</h2>
<div class="table-scroll"><table class="evidence-table">
<caption class="muted">Evidence available as of 2 November 2026</caption>
<thead><tr><th scope="col">Layer</th><th scope="col">Status</th><th scope="col">What the evidence establishes</th></tr></thead>
<tbody>
<tr><td>Synthetic recording/replay</td><td>Tested</td><td><a href="https://github.com/n0711/ROS3-Limassol/pull/70">PR #70</a>: exact generated multi-monitor replay regression coverage</td></tr>
<tr><td>Rover SITL over localhost</td><td>Tested</td><td><a href="https://github.com/n0711/ROS3-Limassol/pull/32">PR #32</a>: simulated ArduPilot blackout detection and recovery; no physical link</td></tr>
<tr><td>Raspberry Pi–Arduino USB</td><td>Limited bench</td><td><a href="https://github.com/n0711/ROS3-Limassol/pull/58">PR #58</a>: one physical USB disconnect/reconnect run; no MAVLink, flight controller or radio</td></tr>
<tr><td>Companion + flight controller + radio</td><td>Not tested</td><td>No repository evidence yet</td></tr>
<tr><td>Vessel/robot field operation</td><td>Not tested</td><td>No repository evidence yet</td></tr>
</tbody></table></div>
<p>The USB result is physical-interface evidence only: it did not include MAVLink, a flight controller, a radio, a vessel or robot in operation. Its recordings are identified by checksums but are not archived in the repository. <strong>Flight-controller integration, radio-link behaviour, physical vessel/robot measurements and field reliability are not yet established.</strong> Consult the repository's <a href="https://github.com/n0711/ROS3-Limassol/blob/main/robotics/ros3_limassol/docs/VALIDATION.md">validation record</a>, <a href="https://github.com/n0711/ROS3-Limassol/blob/main/docs/hardware/ARDUINO_USB_VALIDATION_2026-11-02.md">USB bench record</a> and <a href="https://github.com/n0711/ROS3-Limassol/actions">CI runs</a> for the dated evidence.</p>
<p><a href="https://github.com/n0711/ROS3-Limassol#run-the-prototype">Run the reproducible experiment from source ↗</a></p>
</section>
<section class="panel"><div><p class="eyebrow">Next validation gates</p><h2>What must happen before a field claim</h2></div><div>
<ol><li>Archive raw recordings, manifests, hashes and environment metadata so the reported runs can be independently checked.</li><li>Complete the planned three-Rover SITL isolation test over separate and shared loopback links.</li><li>Validate signed MAVLink through a companion computer, flight controller and physical radio bench before any vessel or robot deployment.</li></ol>
<p>Each result must retain its own scope; passing one gate does not imply that a later gate has passed.</p>
</div></section>
<section class="panel"><div><p class="eyebrow">Related independent projects</p><h2>Explore more PELAOS work</h2></div><div><p><a href="https://github.com/n0711/LearnTwin">LearnTwin</a> is a degree, study and PhD-planning workspace; <a href="https://cysk8clips.com">cysk8clips.com</a> is the Cyprus skateboarding archive and spot guide. They are separate from ROS3-Limassol.</p></div></section>
"##.to_string(),
        crumbs: vec![("ros3-limassol/", "ROS3-Limassol")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn cultural_storytelling_map() -> Page {
    Page {
        slug: "cultural-storytelling-map/",
        title: "Narrify: Cultural Storytelling Map | OpenPELAOS",
        description: "Narrify by Charalampos Nadiotis, Hussain Jamil and Nikitas Savva: community stories, cultural places and a sourced Cyprus place-name pilot.",
        body: r##"
<p class="eyebrow">OpenPELAOS / Culture &amp; community</p>
<h1>Cultural Storytelling Map</h1>
<p class="lead">Narrify connects the stories people tell with the places they belong to.</p>
<p>Explore community accounts through a map, with text, photographs and audio. The existing prototype includes translation and human moderation.</p>
<div class="actions"><a class="button primary" href="https://github.com/hussainJamil24/cultural-storytelling-map">Explore the Narrify source ↗</a></div>
<h2>The team</h2>
<ul>
<li><a href="https://github.com/n0711">Charalampos Nadiotis (n0711)</a></li>
<li><a href="https://github.com/hussainJamil24">Hussain Jamil</a></li>
<li>Nikitas Savva</li>
</ul>
<p>The team repository is hosted under Hussain Jamil's account. The OpenPELAOS extension keeps that project attribution visible.</p>
<h2>The story behind the name</h2>
<p>A proposed place-name layer connects historical names and origin explanations to their sources. Different explanations remain separate, with uncertainty visible.</p>
<p class="status-note">The extension is prepared for review. It is not yet integrated into the Narrify prototype. The examples below are a small sourced pilot, not a classification of every settlement in Cyprus.</p>
<article class="card">
<span class="badge">Name history · Origin not yet added</span>
<h3>Lefkosia / Nicosia</h3>
<p>Nicosia Municipality records Ledra, Lefkothea and Lefkousia among earlier names associated with the city. Historical names alone do not establish a derivation for the present name.</p>
<p><a href="https://www.nicosia.org.cy/en-GB/discover/nicosia/nicosia/names/">Source: Nicosia Municipality</a> · <a href="https://www.geonames.org/146268/nicosia.html">Location: GeoNames</a></p>
</article>
<article class="card">
<span class="badge">Two possible explanations</span>
<h3>Anogyra</h3>
<p>The tourism portal offers two possibilities: a connection to the local anagyris plant, or ano (upper) combined with Gyroi, a former village name. The pilot preserves both explanations.</p>
<p><a href="https://www.visitcyprus.com/discover-cyprus/rural/villages-rural/anogyra-village-2/">Source: Cyprus Deputy Ministry of Tourism</a> · <a href="https://www.geonames.org/146810/anogyra.html">Location: GeoNames</a></p>
</article>
<article class="card">
<span class="badge">Source-reported explanation</span>
<h3>Lazanias</h3>
<p>The tourism portal links the name to Lusignan and a farm associated with the Crusaders. This is presented as the source's explanation, rather than an independent scholarly resolution.</p>
<p><a href="https://www.visitcyprus.com/discover-cyprus/culture/sites-and-monuments/saint-george-s-church-lazanias-village/">Source: Cyprus Deputy Ministry of Tourism</a> · <a href="https://www.geonames.org/146393/lazanias.html">Location: GeoNames</a></p>
</article>
<p class="muted">Sources consulted on 20 October 2026. Settlement points in the proposed layer are approximate reference locations, not historical boundaries. Geographic proximity to a community story does not establish a historical connection.</p>
<p class="muted">The settlement-name infographic that inspired this direction remains separate source material. Its map, boundaries and classifications have not been imported.</p>
"##.to_string(),
        crumbs: vec![("cultural-storytelling-map/", "Cultural Storytelling Map")],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}

fn research() -> Page {
    let ros3 = href("ros3-limassol/");
    let vera = href("vera-mot/");
    let rows = [0.0, 10.0, 50.0, 100.0, 200.0, 300.0, 400.0, MAX_DEPTH_M as f64]
        .iter()
        .map(|&h| {
            format!(
                "<tr><td>{} m</td><td>{}</td><td>{}</td></tr>",
                group(h as u64),
                fixed2(gauge_bar(h)),
                fixed2(window_tonnes(h))
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let max_bar = fixed2(gauge_bar(MAX_DEPTH_M as f64));
    let max_tonnes = fixed2(window_tonnes(MAX_DEPTH_M as f64));
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
            "How much pressure is there at 500 metres?",
            format!(
                "About {max_bar} bar above surface pressure in the constant-density seawater model. On a 10 x 10 cm window with surface pressure inside, that is a net force of about {max_tonnes} tonnes-force."
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
<h1>Computer science, AI and open research</h1>
<p class="lead">Explore reproducible experiments in algorithms, computer vision and distributed robotics software. Each project separates established evidence from open questions.</p>
<section aria-labelledby="complexity-research">
<p class="eyebrow">Computer science / Complexity theory / Research note</p>
<h2 id="complexity-research">P versus NP: a research question</h2>
<p><strong>Research interest:</strong> Charalampos Nadiotis (30 October 2026) is investigating the proposition P = NP. This is a personal research direction, not an established mathematical result or an independently verified proof.</p>
<p><strong>Definitions:</strong> P is the class of decision problems solvable by a deterministic algorithm in polynomial time. NP is the class of decision problems whose proposed solutions can be verified in polynomial time. Whether P equals NP remains an open problem.</p>
<div class="card">
<span class="badge">Unverified research proposition</span>
<h3>What would constitute a proof?</h3>
<p>To establish P = NP, a rigorous polynomial-time algorithm for any NP-complete decision problem, together with a proof of correctness and polynomial running-time bound, would suffice. Numerical experiments, benchmark results, or an unsupported assertion of equality do not resolve the problem.</p>
<p>A complete manuscript, independently checkable argument, and expert scrutiny are required before any solution claim can be reported as verified.</p>
</div>
<p><strong>Primary reference:</strong> <a href="https://www.claymath.org/millennium/p-vs-np/">Clay Mathematics Institute — P vs NP</a>. <a href="https://www.scottaaronson.com/papers/pnp.pdf">Background survey: Scott Aaronson, P versus NP</a>.</p>
</section>

<section aria-labelledby="vera-research-update">
<p class="eyebrow">Computer vision / VERA-MOT / 3 November 2026</p>
<h2 id="vera-research-update">Reliability-gated ReID and protected appearance memory</h2>
<p>An opt-in tracking prototype passed synthetic regression and pinned BoT-SORT integration tests. Its image-quality gate excludes low-quality descriptors from appearance-memory updates while preserving motion and geometric association. No improved tracking metrics have yet been measured on annotated vehicles.</p>
<p><a href="{vera}">Read the XeLaTeX paper, code and validation limits →</a></p>
</section>

<section aria-labelledby="ros3-research-update">
<p class="eyebrow">OpenPELAOS / Evidence update / 2 November 2026</p>
<h2 id="ros3-research-update">ROS3-Limassol validation boundary</h2>
<p>ROS3-Limassol now has synthetic recording/replay evidence, ArduPilot Rover SITL blackout tests over localhost, and one limited Raspberry Pi–Arduino USB disconnection/recovery bench result. The USB test is a physical-interface check, not evidence for MAVLink, a flight controller, a radio link, a deployed vessel or field reliability.</p>
<p><a href="{ros3}">Read the dated results and remaining gaps →</a></p>
</section>

<section aria-labelledby="marine-research">
<p class="eyebrow">Additional research / Marine robotics</p>
<h2 id="marine-research">Underwater systems: depth and navigation</h2>
<p>Explore the depth and pressure model from the surface to {MAX_DEPTH_M} metres. My related ROV research is linked on <a href="{RESEARCHGATE}">ResearchGate</a>; the calculator does not establish a deployed sensor or pressure-rated vehicle.</p>
<h2>PELAOS depth meter · 0–{MAX_DEPTH_M} m</h2>
<p>NOAA: <q>For every 33 feet (10.06 meters) you go down, the pressure increases by one atmosphere.</q> (<a href="{NOAA_PRESSURE}">NOAA Ocean Service</a>)</p>
<div class="card">
<table>
<caption class="muted">0–{MAX_DEPTH_M} m: pressure above surface pressure, and net force on a 10 × 10 cm viewport with surface pressure inside</caption>
<thead><tr><th scope="col">Depth</th><th scope="col">Gauge pressure (bar)</th><th scope="col">Force (tonnes-force)</th></tr></thead>
<tbody>
{rows}
</tbody>
</table>
<div id="pressure-live" data-rho="{RHO}" data-g="{G}" data-max-depth="{MAX_DEPTH_M}" data-initial-depth="{INITIAL_DEPTH_M}"></div>
<p id="pressure-model"><small>Calculated with p = ρgh, constant seawater density {RHO} kg/m³ and g = {G} m/s². This is a calculator, not a live sensor reading or a vehicle depth rating.</small></p>
</div>

<h2>Navigation without GPS</h2>
<p><q>GPS does not work underwater</q>, so a submerged vehicle's position estimate drifts until it can surface for a new fix (<a href="{NOAA_NAVIGATION}">NOAA Ocean Exploration</a>). Deep vehicles therefore combine dead reckoning (inertial sensors, compass, depth and speed) with acoustic positioning, where sound pulses between the vehicle and beacons or a surface vessel give its range and bearing.</p>


</section>

<h2>FAQ</h2>
<dl>
{faq_html}
</dl>
<p><small>ROV and AUV definitions: <a href="{NOAA_AUV_ROV}">NOAA Ocean Service</a>.</small></p>
"#
    );
    Page {
        slug: "research/",
        title: "Computer Science, AI and Open Research | PelaosTech",
        description: "Explore PELAOS research in algorithms, P versus NP, VERA-MOT computer vision and ROS3 distributed robotics software, plus underwater engineering notes.",
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
<p class="lead">An offline tuner for guitar, bass, ukulele and bouzouki, for Android. Development of version 1.0 is finished. The Google Play listing, signed release build and store testing are being prepared.</p>

<h2>How it measures</h2>
<p>RioTuner estimates pitch from multiple string partials and fits a stiff-string model, <code>f<sub>k</sub> = k·f<sub>0</sub>·√(1 + B·k²)</code>, to all of them. Real strings are slightly stiff, so their overtones run sharp. The fit aims to separate string stiffness from pitch, including when the fundamental is weak. The strobe shows phase drift against the target pitch, so the pattern only stands still when the string is in tune.</p>

<p class="status-note">The Google Play link will be added here once the app is live. Until then there is no public download. Synthetic benchmark results do not establish accuracy on a physical phone.</p>
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
<p><small>Synthetic plucked strings with known pitch (B1 to E4, random detuning and string stiffness). This benchmarks the algorithm itself, not other tuner apps. On a phone, results also depend on the microphone and the room.</small></p>
</div>

<h2>Features</h2>
<ul>
<li>16 tunings: guitar (standard, drop D, half and full step down, drop C, DADGAD, open G/D/E, 7-string), bass (4- and 5-string), ukulele, bouzouki (CFAD, DAD), baglamas, plus chromatic mode</li>
<li>Needle and strobe displays</li>
<li>Automatic string detection, or tap a string to lock it</li>
<li>Adjustable in-tune tolerance</li>
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
        description: "RioTuner: an offline Android tuner for guitar, bass, ukulele and bouzouki, coming to Google Play. Synthetic benchmark results inside.",
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
        description: "RioTuner testing preparation: release status, what to check on an Android phone, and how to send useful feedback.",
        body: format!(r#"
<p class="eyebrow">RioTuner / Testing preparation</p>
<h1>Put it through a real practice session.</h1>
<p class="lead">Synthetic signals test the algorithm. Instruments and phones test the product.</p>
<p class="status-note">RioTuner is heading to Google Play, which needs a closed test with opted-in testers before public release. No opt-in link is published here yet. Contact me on LinkedIn if you want to join, and the link will be added when it is available.</p>
<h2>What to check</h2>
<ol><li>When a verified test release is available, install it from the published test link and record the app version.</li><li>Allow microphone access, then deny it once and check that the app explains how to continue.</li><li>Play each open string separately in a quiet room. Check string selection, the displayed note and response to a slow tuning change.</li><li>Compare needle and strobe displays, then repeat with background noise and a weaker low note.</li><li>Switch apps, lock and unlock the phone, and reopen the tuner. Microphone use should stop in the background and recover correctly.</li><li>Change reference pitch and tuning, restart the app and check that your settings are retained.</li></ol>
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
    let hosting_notice = if HOSTING == "vercel" {
        "<p>This site is delivered through Vercel. Hosting involves handling network requests, including IP addresses and request metadata. See <a href=\"https://vercel.com/legal/privacy-notice\">Vercel's Privacy Notice</a> for its privacy practices. PelaosTech adds no analytics or tracking scripts.</p>".to_string()
    } else {
        format!("<p>The site is hosted on GitHub Pages. GitHub states that <q>when a GitHub Pages site is visited, the visitor's IP address is logged and stored for security purposes</q> (<a href=\"{GITHUB_PAGES_PRIVACY}\">GitHub Docs</a>). PelaosTech does not receive or use those logs. GitHub's handling of them is covered by the GitHub General Privacy Statement.</p>")
    };
    let body = format!(
        r#"
<h1>Website privacy</h1>
<p>This website has no cookies, no analytics, no tracking and no forms. Its only script is the depth slider on the research page, which runs entirely in your browser.</p>
{hosting_notice}
<p>The RioTuner app has its own <a href="{policy}">privacy policy</a>.</p>
"#,
        policy = href("riotuner/privacy/"),
    );
    Page {
        slug: "privacy/",
        title: "Website privacy | PelaosTech",
        description: "The PelaosTech website adds no cookies, analytics or forms. Hosting-provider information and the separate RioTuner app privacy policy.",
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

const MANIFEST: &str = r#"{
  "id": "__BASE__/",
  "name": "PelaosTech - Software and Research",
  "short_name": "PelaosTech",
  "description": "Independent software, computer science, AI and research projects.",
  "start_url": "__BASE__/",
  "scope": "__BASE__/",
  "display": "standalone",
  "background_color": "#070c18",
  "theme_color": "#070c18",
  "icons": [
    {"src": "__BASE__/icon-192.png", "sizes": "192x192", "type": "image/png", "purpose": "any"},
    {"src": "__BASE__/icon-512.png", "sizes": "512x512", "type": "image/png", "purpose": "any maskable"}
  ]
}
"#;

// Registration stays external: the site CSP does not permit inline scripts.
const PWA_REGISTRATION: &str = r#"if ('serviceWorker' in navigator && window.isSecureContext) {
  window.addEventListener('load', () => {
    navigator.serviceWorker.register('__BASE__/sw.js', { scope: '__BASE__/' })
      .catch(() => { /* Online pages remain fully usable. */ });
  });
}
"#;

// Cache only this portfolio's public, same-origin pages and assets.
// Network-first navigation avoids hiding newly published research behind stale HTML.
const SERVICE_WORKER: &str = r#"const VERSION = 'pelaos-pwa-20261105';
const BASE = '__BASE__/';
const PRECACHE = [
  '', 'services/', 'research/', 'vera-mot/', 'ros3-limassol/',
  'cultural-storytelling-map/', 'riotuner/', 'riotuner/testing/',
  'riotuner/privacy/', 'privacy/', 'app.js', 'pwa.js',
  'favicon.svg', 'icon-192.png', 'icon-512.png', 'manifest.webmanifest'
].map(path => BASE + path);

self.addEventListener('install', event => {
  event.waitUntil(
    caches.open(VERSION)
      .then(cache => cache.addAll(PRECACHE))
      .then(() => self.skipWaiting())
  );
});

self.addEventListener('activate', event => {
  event.waitUntil(
    caches.keys()
      .then(keys => Promise.all(keys
        .filter(key => key.startsWith('pelaos-pwa-') && key !== VERSION)
        .map(key => caches.delete(key))))
      .then(() => self.clients.claim())
  );
});

self.addEventListener('fetch', event => {
  const request = event.request;
  if (request.method !== 'GET') return;
  const url = new URL(request.url);
  if (url.origin !== self.location.origin || !url.pathname.startsWith(BASE)) return;
  if (request.mode === 'navigate') {
    event.respondWith(
      fetch(request)
        .then(response => {
          if (response.ok) {
            const copy = response.clone();
            event.waitUntil(caches.open(VERSION).then(cache => cache.put(request, copy)));
          }
          return response;
        })
        .catch(async () => (await caches.match(request, { ignoreSearch: true }))
          || (await caches.match(BASE)))
    );
    return;
  }
  event.respondWith(
    caches.match(request).then(cached => cached || fetch(request).then(response => {
      if (response.ok && response.type === 'basic') {
        const copy = response.clone();
        event.waitUntil(caches.open(VERSION).then(cache => cache.put(request, copy)));
      }
      return response;
    }))
  );
});
"#;

fn pages() -> Vec<Page> {
    vec![home(), services(), research(), vera_mot(), ros3_limassol(), cultural_storytelling_map(), riotuner(), riotuner_testing(), riotuner_privacy(), site_privacy(), not_found()]
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
    // robots.txt is effective at the domain root (root-path Vercel builds). GitHub Pages
    // project URLs place this file in a subdirectory: there it is informational only.
    files.push(File { path: "robots.txt".into(), contents: format!("User-agent: *\nAllow: /\nSitemap: {SITE}/sitemap.xml\n") });
    files.push(File {
        path: "llms.txt".into(),
        contents: format!(
            "# PelaosTech\n\n> Independent software, computer science, AI and open research, including RioTuner, VERA-MOT and ROS3-Limassol.\n\n- [Research: algorithms, AI and robotics]({}): P versus NP (unverified), VERA-MOT, ROS3-Limassol, plus sourced underwater pressure and navigation notes\n- [ROS3-Limassol]({}): receiver-arrival liveness evidence from synthetic replay, Rover SITL over localhost and a limited Raspberry Pi–Arduino USB bench test; flight-controller, radio-link and field validation pending\n- [VERA-MOT]({}): quality-gated appearance matching verified only on synthetic pinned-tracker tests; vehicle identity performance remains unmeasured\n- [RioTuner]({}): offline Android instrument tuner, Google Play release in preparation, with synthetic benchmark results\n- [RioTuner privacy policy]({})\n",
            url("research/"),
            url("ros3-limassol/"),
            url("vera-mot/"),
            url("riotuner/"),
            url("riotuner/privacy/")
        ),
    });
    files.push(File { path: "favicon.svg".into(), contents: FAVICON.to_string() });
    files.push(File { path: "manifest.webmanifest".into(), contents: MANIFEST.replace("__BASE__", BASE) });
    files.push(File { path: "pwa.js".into(), contents: PWA_REGISTRATION.replace("__BASE__", BASE) });
    files.push(File { path: "sw.js".into(), contents: SERVICE_WORKER.replace("__BASE__", BASE) });
    files
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html() -> Vec<(String, String)> {
        site().into_iter().filter(|f| f.path.ends_with(".html")).map(|f| (f.path, f.contents)).collect()
    }

    #[test]
    fn deployment_urls_and_hosting_match_configuration() {
        validate_config().unwrap();
        for page in pages() {
            let rendered = layout(&page);
            assert!(rendered.contains(&format!("<link rel=\"canonical\" href=\"{}\">", url(page.slug))));
            assert!(rendered.contains(&format!("href=\"{BASE}/favicon.svg\"")));
            assert!(rendered.contains(&format!("Hosted on {}.", hosting_name())));
            if page.script {
                assert!(rendered.contains(&format!("src=\"{BASE}/app.js\"")));
            }
        }
        let privacy = site_privacy().body;
        assert_eq!(privacy.contains("https://vercel.com/legal/privacy-notice"), HOSTING == "vercel");
    }

    #[test]
    fn pwa_assets_use_the_site_base_and_remain_external_scripts() {
        let files = site();
        let manifest = &files.iter().find(|f| f.path == "manifest.webmanifest").unwrap().contents;
        let registration = &files.iter().find(|f| f.path == "pwa.js").unwrap().contents;
        let worker = &files.iter().find(|f| f.path == "sw.js").unwrap().contents;
        assert!(manifest.contains(&format!(r#""start_url": "{BASE}/""#)));
        assert!(manifest.contains(&format!(r#""scope": "{BASE}/""#)));
        assert!(manifest.contains(&format!("{BASE}/icon-192.png")));
        assert!(manifest.contains(&format!("{BASE}/icon-512.png")));
        assert!(registration.contains(&format!("{BASE}/sw.js")));
        assert!(worker.contains(&format!("const BASE = '{BASE}/'")));
        assert!(worker.contains("url.origin !== self.location.origin"));
        for page in pages() {
            let document = layout(&page);
            assert!(document.contains(&format!(r#"rel="manifest" href="{BASE}/manifest.webmanifest""#)));
            assert!(document.contains(&format!(r#"src="{BASE}/pwa.js""#)));
        }
    }

    #[test]
    fn release_status_does_not_claim_unverified_phone_or_store_testing() {
        for file in site() {
            let content = file.contents.to_lowercase();
            assert!(!content.contains("tested on android phones"), "{}", file.path);
            assert!(!content.contains("tested on real phones"), "{}", file.path);
            assert!(!content.contains("going through the google play closed test"), "{}", file.path);
        }
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
        assert!(summary.contains("synthetic benchmark results"));
        assert!(!summary.contains("measured accuracy"));
        assert!(summary.contains("ROS3-Limassol"));
    }

    #[test]
    fn ros3_has_verified_credits_and_field_caveat() {
        let rendered = layout(&ros3_limassol());
        assert!(rendered.contains("Christos Paparistodemou"));
        assert!(rendered.contains("Charalampos Nadiotis"));
        assert!(rendered.contains("physical vessel/robot measurements"));
        assert!(rendered.contains("USB result is physical-interface evidence only"));
        assert!(rendered.contains("Companion + flight controller + radio"));
        assert!(rendered.contains("No repository evidence yet"));
        assert!(rendered.contains("not an official ROS distribution"));
        assert!(rendered.contains("not peer-reviewed"));
        let landing = layout(&home());
        assert!(landing.contains(&format!("href=\"{}\"", href("ros3-limassol/"))));
        let summary = site().into_iter().find(|f| f.path == "llms.txt").unwrap().contents;
        assert!(summary.contains(&url("ros3-limassol/")));
    }

    #[test]
    fn vera_mot_research_page_links_working_paper_and_limits() {
        let rendered = layout(&vera_mot());
        assert!(rendered.contains("not a peer-reviewed paper"));
        assert!(rendered.contains("Vehicle identity benchmark"));
        assert!(rendered.contains("Not established"));
        assert!(rendered.contains("VERA_MOT_Implementation_Note.tex"));
        assert!(rendered.contains("VERA_MOT_Implementation_Note.pdf"));
        assert!(rendered.contains("does not detect occlusion"));
        assert!(rendered.contains("not been uploaded to ResearchGate"));
        assert!(layout(&home()).contains(&format!("href=\"{}\"", href("vera-mot/"))));
        assert!(layout(&research()).contains(&format!("href=\"{}\"", href("vera-mot/"))));
        let sitemap = site().into_iter().find(|f| f.path == "sitemap.xml").unwrap().contents;
        assert!(sitemap.contains(&format!("<loc>{}</loc>", url("vera-mot/"))));
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
        assert_eq!(sitemap.matches("<url>").count(), pages().iter().filter(|p| !p.noindex).count());
        assert!(!sitemap.contains("404"));
    }

    #[test]
    fn seo_pages_are_unique_and_sitemap_is_consistent() {
        let pages = pages();
        let mut titles = std::collections::HashSet::new();
        let mut descriptions = std::collections::HashSet::new();
        let sitemap = site().into_iter().find(|f| f.path == "sitemap.xml").unwrap().contents;
        for page in &pages {
            assert!(titles.insert(page.title), "duplicate page title: {}", page.title);
            assert!(descriptions.insert(page.description), "duplicate page description: {}", page.description);
            let expected = format!("<loc>{}</loc>", url(page.slug));
            assert_eq!(sitemap.contains(&expected), !page.noindex, "sitemap mismatch: {}", page.slug);
            let rendered = layout(page);
            assert!(rendered.contains(r#"<meta property="og:locale" content="en_GB">"#));
            assert!(rendered.contains(r#"<meta name="twitter:title" content=""#));
            assert!(rendered.contains(r#"<meta name="twitter:description" content=""#));
        }
        let proposed = layout(&services());
        assert!(proposed.contains("Small-business websites in Cyprus"));
        assert!(proposed.contains("not a live ordering or payment page"));
        assert!(layout(&home()).contains(&href("services/")));
    }

    #[test]
    fn pressure_figures() {
        assert_eq!(gauge_bar(0.0), 0.0);
        assert_eq!(window_tonnes(0.0), 0.0);
        assert!((gauge_bar(500.0) - 50.25908125).abs() < 1e-8);
        assert!((window_tonnes(500.0) - 5.125).abs() < 1e-8);
        assert_eq!(fixed2(window_tonnes(500.0)), "5.13");
        assert_eq!(group(11000), "11,000");
    }
}
