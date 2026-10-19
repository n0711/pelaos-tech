# Product decisions — 19 October 2026

This is an evidence summary and decision record. Prices and validation targets are experiments, not forecasts. No customer, revenue or download counts have been invented.

## Order of work

1. **PELAOS services:** a tightly scoped website job can be sold without building a new SaaS platform. Use existing work as proof and an existing contact channel. Test the scope in [SERVICE_OFFER.md](SERVICE_OFFER.md).
2. **cysk8clips:** keep the useful archive public. Test labelled sponsorship and voluntary support before charging for a map whose paid-access system is not implemented.
3. **RioTuner:** prepare a paid app, but establish physical-device behaviour, release signing and store readiness first. Synthetic accuracy tests do not establish real-phone accuracy.
4. **LearnTwin:** improve first-use and backup reliability; validate optional setup assistance. Do not present the local planner as a hosted, authenticated school platform.

## Website decisions

The Rust generator and Dart enhancement remain. The homepage now gives each project a use case, an explicit availability state and a next step. Research remains identified as research. RioTuner's description and metadata no longer promise exact or measured real-phone accuracy.

The public website stays informational because [GitHub Pages limits](https://docs.github.com/en/pages/getting-started-with-github-pages/github-pages-limits) restrict using Pages primarily for commercial transactions or SaaS. A future commercial website needs a host whose terms support that use. A custom domain alone does not solve this.

## Validation rules

- A test result is evidence only for the code and environment tested.
- A link to Patreon, Revolut or a store does not prove ownership, payout configuration or a completed purchase.
- No checkout is enabled for an undelivered product.
- User interviews, quotes and payments are separate milestones.
- Avoid new framework migrations unless they remove a measured product constraint.

## First 14 days

These are suggested targets, not scheduled messages or commitments.

- Discuss the landing-page offer with five relevant businesses after outreach is authorised; aim to learn whether one will accept a written quote.
- Ask three relevant local businesses about a defined cysk8clips placement; collect objections and requested evidence.
- Put RioTuner through documented real-device sessions before using accuracy as a sales claim.
- Ask three students to complete LearnTwin setup and return a week later.

Measure conversations, accepted scopes, actual payments, delivery hours and repeated use. Keep personal lead information outside public repositories.
