# Licence and attribution inventory
26 October 2026 | Draft v0.1 | Records current notices; no relicensing

## Inventory

| Artefact | Current recorded terms / evidence | Reuse position recorded by those sources | Open point / next documentary check |
| --- | --- | --- | --- |
| ROS3-Limassol Python package | Apache-2.0 in package.xml and package LICENSE [R2], [R3] | Package distribution is labelled Apache-2.0; retain its actual licence and applicable notices | Review contributor ownership and any third-party material at a release boundary |
| ROS3 experimental Rust core | Apache-2.0 in Cargo.toml and its LICENSE [R4], [R5] | Same software licence label; manuscript policy is recorded separately | Inventory dependency notices in a distribution; this task does not audit all dependencies |
| LearnTwin material inherited in ROS3 repository | Root MIT licence, copyright 2025 Charalampos Nadiotis and LearnTwin contributors; scope identified in README [R1], [R6] | The MIT text grants the listed software/documentation permissions subject to its notice condition | Preserve contributor credit and distinguish inherited files from robotics package files |
| ROS3 ResearchGate preprint edition dated 25 October 2026 and its XeLaTeX source | Civilian-Use Licence (No Military Use), v1.0 [R1], [R7] | The named edition permits the specified civilian reuse, including commercial civilian activity; its text excludes military-purpose reuse, referenced third-party works and separately licensed software | Identify this exact edition in copies; preserve author, title, licence, change notices and initial-idea credit |
| Earlier ROS3 report and arXiv-preparation edition | Separate artefacts linked from the README [R1] | Their file-level notices were not inspected for this matrix | Inspect each edition before asserting it shares the later preprint terms; do not copy one edition's notice across all editions |
| General project documentation | README licence sections, package notices and file-level headers define the reviewed scope [P1], [R1] | No blanket permission is inferred for every document in every repository | Record the applicable notice per document; check third-party excerpts and sources individually |
| PELAOS website source and media | Reviewed repository tree has no root LICENSE; README ends with a PELAOS copyright notice [P1] | An explicit reusable website-source grant was not identified in this audit | Owner chooses any future grant after checking authored code, illustrations and credited third-party content; licence choice is unresolved |
| Reels, cover artwork and other media | Individual media rights and embedded assets were not audited in this task | Attribution, music rights and reuse permissions remain per asset | Make an asset ledger before any reuse; do not infer rights from a PELAOS/OpenPELAOS label |

## Labels to keep distinct

- Programme mission: civilian research is the proposed OpenPELAOS direction.
- Existing software: Apache-2.0 and MIT notices apply to their identified materials.
- Named manuscript: the custom civilian-use notice applies to its specified edition.
- New charter: a planning draft that grants no new rights.

The reviewed manuscript licence explicitly says its military-use restriction does not amend separately licensed software or change previously granted software rights. This inventory follows that recorded boundary. It makes no enforceability finding.

## Attribution record

- Charalampos Nadiotis: current ROS3 project lead, implementation and manuscript roles as recorded by the project.
- Christos Paparistodemou: initial ROS3-Limassol project idea.
- LearnTwin contributors: retain their existing licence and recorded credits.
- Narrify: the PELAOS README records Charalampos Nadiotis, Hussain Jamil and Nikitas Savva; inclusion in a portfolio does not establish sole ownership.

## Completion criteria

The inventory distinguishes software, documents, manuscript editions and media, with unresolved entries visible. Before any future licence change, record the material, version, rights holders, previous notices and the exact proposed change. This pack changes none of those notices.

[P1]: https://github.com/n0711/pelaos-tech/blob/840e063935985ca3cdbf49707f9233e40a33ec0f/README.md
[R1]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/README.md
[R2]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/robotics/ros3_limassol/package.xml
[R3]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/robotics/ros3_limassol/LICENSE
[R4]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/robotics/ros3_core/Cargo.toml
[R5]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/robotics/ros3_core/LICENSE
[R6]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/LICENSE
[R7]: https://github.com/n0711/ROS3-Limassol/blob/2f071da0ec5891ef6dc42c02fa5fab3b9cdbbefb/docs/papers/MANUSCRIPT_LICENCE.md



## Related planning drafts

- [Project map](PROJECT_MAP.md)
- [OpenPELAOS charter](OPENPELAOS_CHARTER.md)
