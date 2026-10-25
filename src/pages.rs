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
pub const UPDATED: &str = "2026-10-25";

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
@media(max-width:720px){header,main,footer{padding-left:18px;padding-right:18px}.project-grid,.panel{grid-template-columns:1fr}.hero{padding:8px 0 28px}.lead{font-size:18px}.project{padding:22px}nav a{padding:0 8px}table{font-size:14px}}

"#;

fn layout(p: &Page) -> String {
    let canonical = url(p.slug);
    let nav = [("", "Home"), ("research/", "Research"), ("ros3-limassol/", "ROS3"), ("riotuner/", "RioTuner")]
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
<a href="{research}">Research</a><a href="{riotuner}">RioTuner</a><a href="{privacy}">Website privacy</a><a href="{CONTACT}">Contact</a><a href="{REVOLUT}">Support ↗</a>
<p>© 2026 PelaosTech. Last updated <time datetime="{UPDATED}">25 October 2026</time>. Hosted on {hosting}.</p>
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
        hosting = hosting_name(),
        body = p.body.trim(),
    )
}

fn home() -> Page {
    let body = format!(
        r##"
<section class="hero">
<p class="eyebrow">PELAOS / Independent software &amp; research / Cyprus</p>
<h1>Useful software.<br>Built with a purpose.</h1>
<p class="lead">Independent and team projects by Charalampos Nadiotis, connecting music, learning and local culture with practical engineering.</p>
<div class="actions"><a class="button primary" href="#projects">Explore the projects ↓</a><a class="button" href="{LINKEDIN}">Meet the maker ↗</a></div>
</section>
<section id="projects" aria-labelledby="project-heading">
<p class="eyebrow">Selected projects</p><h2 id="project-heading">Different interests. Things you can use.</h2>
<div class="project-grid">
<article class="card project"><span class="badge">Android · Coming to Google Play</span><h3>RioTuner</h3><p>An offline instrument tuner with needle and strobe displays for guitar, bass, ukulele and bouzouki. Development is finished and the Google Play release is being prepared.</p><a href="{riotuner}">Explore RioTuner →</a></article>
<article class="card project"><span class="badge">Live · Community website</span><h3>cysk8clips</h3><p>Cyprus skateboarding clips, full parts, event stories and a public spot guide. A record of the people and places behind the scene.</p><a href="https://cysk8clips.com">Visit the skate archive ↗</a></article>
<article class="card project"><span class="badge">OpenPELAOS · Team prototype</span><h3>Cultural Storytelling Map</h3><p>Narrify connects community stories with places. Co-developed by Charalampos Nadiotis, Hussain Jamil and Nikitas Savva.</p><a href="{narrify}">Explore Narrify →</a></article>
<article class="card project"><span class="badge">Open source · Local planner</span><h3>LearnTwin</h3><p>Track degree progress, study tasks, thesis milestones and PhD applications. Records stay in your browser; export a backup to keep them.</p><a href="https://github.com/n0711/LearnTwin#start-the-planner">Try LearnTwin from source ↗</a></article>
<article class="card project wide"><span class="badge">OpenPELAOS · ROS 2 Humble / Jazzy</span><h3>ROS3-Limassol</h3><p>Telemetry arrival-liveness monitoring for robots with intermittent connectivity. Record and replay blackouts, detect stale updates and study recovery using synthetic scenarios; physical field validation is pending. Original idea: Christos Paparistodemou. Project lead and manuscript author: Charalampos Nadiotis. This independent prototype is not an official ROS release.</p><a href="{ros3}">Explore ROS3 research →</a></article>
<article class="card project wide"><span class="badge">Research code · In progress</span><h3>VERA-MOT</h3><p>Multi-object tracking for drone video. The question: can unreliable appearance embeddings be stopped from corrupting identity memory when the camera moves, targets shrink and vehicles hide behind each other? The detector and a frozen BoT-SORT baseline are done (held-out detector AP 50.1, tracker HOTA 43.3). ReID training and the VERA association method come next.</p><a href="https://github.com/n0711/vera-mot">Read the VERA-MOT source ↗</a></article>
</div>
</section>
<section class="panel"><div><p class="eyebrow">Research notebook</p><h2>Curiosity goes deeper.</h2></div><div><p>Marine robotics is my main research direction. I have published my ROV work on <a href="{RESEARCHGATE}">ResearchGate</a> and plan to take it further as doctoral research. The notes cover pressure, navigation and why operating underwater is hard. The deep-submergence vehicle is a concept, not a deployed product.</p><a href="{research}">Explore the deep-sea notes →</a></div></section>
<section class="panel"><div><p class="eyebrow">The person behind PELAOS</p><h2>From Cyprus, with curiosity.</h2></div><div><p>I’m Charalampos Nadiotis, a robotics engineer and Computer Science student at Frederick University. I was previously a Robotics Engineer at ADDITESS, working across the full R&amp;D cycle of field robotics, from research and software to integration and field testing. This is my project portfolio: what is available, what is being tested and what I’m exploring next.</p><p><a href="https://github.com/n0711">GitHub</a> · <a href="{RESEARCHGATE}">ResearchGate</a> · <a href="{LINKEDIN}">Contact on LinkedIn</a></p></div></section>
<section class="panel" id="support"><div><p class="eyebrow">Support</p><h2>Keep the projects going.</h2></div><div><p>The projects here are free and built in my own time. If one of them is useful to you, you can support further development with a one-off contribution via Revolut. Every contribution is voluntary and does not buy access, features or support.</p><a class="button primary" href="{REVOLUT}">Support via Revolut ↗</a></div></section>
"##,
        research = href("research/"),
        riotuner = href("riotuner/"),
        narrify = href("cultural-storytelling-map/"),
        ros3 = href("ros3-limassol/"),
    );
    Page {
        slug: "",
        title: "PELAOS | Software, music & community projects",
        description: "ROS3-Limassol telemetry monitoring, LearnTwin, cysk8clips, RioTuner and other OpenPELAOS software and robotics research by Charalampos Nadiotis in Cyprus.",
        body,
        crumbs: vec![],
        extra_ld: vec![],
        script: false,
        noindex: false,
    }
}



fn ros3_limassol() -> Page {
    Page {
        slug: "ros3-limassol/",
        title: "ROS3-Limassol | OpenPELAOS Robotics Research",
        description: "Explore ROS3-Limassol: ROS 2 telemetry-liveness recording and replay, communication blackouts, research credits and validation limits.",
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
<p>The project documents unit tests, synthetic ROS message recording and replay, and CI configurations for both ROS 2 distributions. The deterministic example transitions through WAITING, HEALTHY, STALE and HEALTHY. <strong>Physical vessel/robot measurements, network timing and field reliability are not yet established.</strong> Consult the repository's <a href="https://github.com/n0711/ROS3-Limassol/blob/main/robotics/ros3_limassol/docs/VALIDATION.md">validation record</a> and <a href="https://github.com/n0711/ROS3-Limassol/actions">CI runs</a> for the dated evidence.</p>
<p><a href="https://github.com/n0711/ROS3-Limassol#run-the-prototype">Run the reproducible experiment from source ↗</a></p>
</section>
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
<h1>Marine robotics: depth to {MAX_DEPTH_M} m</h1>
<p class="lead">Explore depth and pressure from the surface to {MAX_DEPTH_M} metres, with sources.</p>
<p class="status-note">These notes support my research direction in marine robotics. My ROV work is published on <a href="{RESEARCHGATE}">ResearchGate</a>, and I plan to continue it at doctoral level.</p>

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

<h2>FAQ</h2>
<dl>
{faq_html}
</dl>
<p><small>ROV and AUV definitions: <a href="{NOAA_AUV_ROV}">NOAA Ocean Service</a>.</small></p>
"#
    );
    Page {
        slug: "research/",
        title: "Marine robotics: depth and pressure | PelaosTech",
        description: "PELAOS depth meter from 0 to 500 m, seawater pressure and viewport force, ROV vs AUV, and underwater navigation. Figures with NOAA sources.",
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

fn pages() -> Vec<Page> {
    vec![home(), research(), ros3_limassol(), cultural_storytelling_map(), riotuner(), riotuner_testing(), riotuner_privacy(), site_privacy(), not_found()]
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
    // Crawlers read these files at the domain root, including root-path Vercel builds.
    files.push(File { path: "robots.txt".into(), contents: format!("User-agent: *\nAllow: /\nSitemap: {SITE}/sitemap.xml\n") });
    files.push(File {
        path: "llms.txt".into(),
        contents: format!(
            "# PelaosTech\n\n> Marine robotics research (deep-submergence ROV concept) and precision software (RioTuner, an instrument tuner).\n\n- [Research: pressure and navigation]({}): pressure at depth with NOAA sources, ROV vs AUV, navigation without GPS\n- [ROS3-Limassol]({}): experimental receiver-level telemetry arrival-liveness monitoring, ROS 2 Humble/Jazzy adapters and deterministic synthetic blackout/recovery tests; physical field validation pending\n- [RioTuner]({}): offline Android instrument tuner, Google Play release in preparation, with synthetic benchmark results\n- [RioTuner privacy policy]({})\n",
            url("research/"),
            url("ros3-limassol/"),
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
        assert!(rendered.contains("Physical vessel/robot measurements"));
        assert!(rendered.contains("not an official ROS distribution"));
        assert!(rendered.contains("not peer-reviewed"));
        let landing = layout(&home());
        assert!(landing.contains(&format!("href=\"{}\"", href("ros3-limassol/"))));
        let summary = site().into_iter().find(|f| f.path == "llms.txt").unwrap().contents;
        assert!(summary.contains(&url("ros3-limassol/")));
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
        assert_eq!(sitemap.matches("<url>").count(), 8);
        assert!(!sitemap.contains("404"));
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
