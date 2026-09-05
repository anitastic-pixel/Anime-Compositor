# Evidence register and research limitations

> **NOTE for version 0.3.** This register is unchanged and remains accurate as a record of what was checked and when. Two entries are no longer load-bearing: the Qt licensing source, because Qt was not selected (ADR-004), and the competitive-tool sources, because the build-versus-extend comparison was closed on preference rather than evidence (D-15).
>
> Nothing here has been re-verified since it was written. Treat every dated claim as of its date.


Version 0.2 | 2026-09-04 | Proposed baseline

## Source use

Sources were checked on 4 September 2026. Some search results carry changing or inconsistent publication metadata; access date is recorded rather than inferred release history. URLs identify primary sources. Statements outside these sources are proposals or explicitly marked inferences.

S-01 / OLM Digital, OpenTools. https://www.olm.co.jp/rd/categories/opentools

Supports: named effect categories; OLM's account of production use; statement that some OpenTools are offered under Apache 2.0. Does not establish licenses for every component, available source for each item or industry-wide adoption. Used in 09, 10 and 16.

S-02 / Adobe, Understanding the expression language. https://helpx.adobe.com/after-effects/desktop/work-with-expressions/expression-basics/expression-language.html

Supports: JavaScript basis with additional built-in objects. Does not establish that a generic JavaScript runtime can execute AE expressions faithfully. Used in 09.

S-03 / OpenColorIO, Authoring Configurations. https://opencolorio.readthedocs.io/en/latest/guides/authoring/authoring.html

Supports: configurable roles and color transforms. Does not select this project's color pipeline or dependency version. Used in 08.

S-04 / U.S. Copyright Office, Circular 61 and What Is Copyright. https://www.copyright.gov/circs/circ61.pdf ; https://www.copyright.gov/what-is-copyright/

Supports: distinction between protected expression and functional ideas/methods. Does not provide project-specific legal clearance or resolve all other intellectual-property questions. Used in 10.

S-05 / Qt, Obligations of the GPL and LGPL. https://www.qt.io/development/open-source-lgpl-obligations

Supports: commercial/open-source paths and obligation review. Vendor-authored and commercially interested; exact module/license/build must be examined. Used in 06 and 10.

---

## Additional primary sources

S-06 / FFmpeg, License and Legal Considerations. https://www.ffmpeg.org/legal.html

Supports: LGPL base with optional GPL components affecting license outcome; configuration review is necessary. Does not clear this project's codecs, patents or distribution. Used in 10.

S-07 / PSOFT, Applying PSOFT anti-aliasing and About the After Effects Render Engine. https://docs.psoft.co.jp/ant100w/en/latest/operation/AA_effect_asign.html ; https://docs.psoft.co.jp/ant100w/en/latest/install/AA_render_engine.html

Supports: an AE effect and documented render-engine operation. Does not establish superiority, source license or native host portability. Used in 16.

S-08 / bryful, F-s-PluginsProjects repository. https://github.com/bryful/F-s-PluginsProjects

Supports: author's published AE plugin collection. Exact revision, individual licenses and SDK dependencies need examination before copying or building. Used in 16.

S-09 / OpenToonz project and documentation. https://opentoonz.github.io/e/index.html ; https://opentoonz.readthedocs.io/en/latest/applying_special_fx.html

Supports: Xsheet/timeline availability and documented effects workflow. Does not promise interchange with this project. Used in 16.

S-10 / Blender Manual, Compositing Introduction. https://docs.blender.org/manual/en/5.0/compositing/introduction.html

Supports: node-based compositing. Version-specific reference; not a current-feature audit or a claim about comparative suitability. Used in 16.

## Context provenance and open research

C-01 / Supplied project excerpts, dated 4 September 2026: user requests an AE-alternative compositor, 2D/traditional animation support, effects/expressions consideration, legal care, anime production-tool research and anti-bloat planning. Full earlier assistant answers were not available. New detailed requirements therefore remain PROPOSED.

OPEN evidence: artist interviews, benchmark measurements, chosen hardware, source/dependency audits, minimum system support, total effort, comparative workflow tests and legal review. No report in this pack should be read as completing those investigations.

When adding evidence, record the exact claim, primary source, access date, version where relevant, limitations and the decision it changes. Keep facts distinct from recommendations and vendor statements distinct from independently tested outcomes.

## Version 0.2 additional primary sources

S-11 / Natron project website. https://natrongithub.github.io/

Supports: Natron identifies itself as an open-source node-based compositor and lists OpenFX plug-in support, tracking, roto, keying, curve/dope-sheet editing and headless rendering. Does not establish current market adoption or suitability for this project's layer-first UX. Used in 30.

S-12 / Blackmagic Design, DaVinci Resolve - Fusion. https://www.blackmagicdesign.com/products/davinciresolve/fusion

Supports: Fusion is built into DaVinci Resolve; Blackmagic describes a node-based workflow with 2D/3D tools, masks and animation editors, and provides a free Resolve download alongside Studio. Vendor-authored promotional source; does not establish comparative workflow efficiency. Used in 30.

S-13 / Foundry, Nuke Non-commercial. https://www.foundry.com/products/nuke-family/non-commercial

Supports: free non-commercial availability and stated restrictions including HD output and selected disabled capabilities. Terms/features can change and must be rechecked before relying on them. Used in 30.

S-14 / Adobe, Creating layers in After Effects; Cameras, lights, and points of interest. https://helpx.adobe.com/after-effects/desktop/work-with-layers/create-layers/creating-layers.html ; https://helpx.adobe.com/after-effects/desktop/work-with-layers/camera-layer/cameras-lights-points-interest.html

Supports: AE's layer-oriented composition model, adjustment/precomposition/3D layer concepts and camera behavior. Does not authorize copying proprietary assets/code or establish a required compatibility target. Used in 30.

S-15 / CELSYS, Business overview (RETAS STUDIO). https://www.celsys.com/businessfield/

Supports: CELSYS identifies RETAS STUDIO as commercial animation-production software and makes a broad adoption claim regarding Japanese animation companies. This is a vendor statement with promotional/historical bias and is not independently verified current adoption. Used in 31.

S-16 / OpenToonz documentation, Working in Xsheet/Timeline. https://opentoonz.readthedocs.io/en/latest/working_in_xsheet.html

Supports: exposing animation levels into Xsheet/Timeline cells/columns and related exposure workflow concepts. Does not imply direct file interchange with this project. Used in 31.

S-17 / PSOFT Online Manuals; anti-aliasing for AE documentation. https://docs.psoft.co.jp/ ; https://docs.psoft.co.jp/ant100w/en/latest/operation/AA_effect_asign.html ; https://docs.psoft.co.jp/ant100w/en/latest/install/AA_render_engine.html

Supports: PSOFT currently publishes manuals for multiple AE-oriented tools including anti-aliasing/CelFX/CelMX, and documents anti-aliasing as an AE effect with render-engine behavior. Does not establish source reuse rights, superiority or studio-wide usage. Used in 31.

