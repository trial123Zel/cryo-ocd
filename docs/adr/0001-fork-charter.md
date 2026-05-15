# ADR-0001: Fork charter & relationship to upstream

- **Status:** Accepted
- **Date:** 2026-05-15

## Context

[`cryo`](https://github.com/paradigmxyz/cryo) is a widely used tool for
extracting blockchain data to Parquet, CSV, JSON, and Python dataframes. Its
last release (`0.2.0`) was published in August 2023; its `main` branch took only
sporadic changes through early 2025 and then went quiet. At the time of this
fork the upstream repository had dozens of open issues and pull requests with no
maintainer response, including bug fixes contributed by the community.

The tool remains genuinely useful to data analysts. The gap is maintenance, not
design.

## Decision

cryo-ocd is a **community-maintained fork** of `paradigmxyz/cryo`. The charter:

1. **Selective import, not mirroring.** The upstream issue/PR tracker is *not*
   mirrored, closed, commented on, or adjudicated. Worthwhile upstream issues
   and pull requests are evaluated and selectively re-created as cryo-ocd
   issues. Out-of-scope items are simply left alone on the upstream repository.
2. **Attribution is preserved.** Adopted community work keeps its original
   authorship via cherry-picked commits or `Co-Authored-By:` trailers, and is
   credited in `CHANGELOG.md` and `ACKNOWLEDGEMENTS.md`.
3. **Licensing is preserved.** `LICENSE-MIT` and `LICENSE-APACHE`, and the
   original copyright notices, remain unchanged. New work is contributed under
   the same dual license so the project stays freely forkable.
4. **Not affiliated with Paradigm.** This is stated in `README.md` and `NOTICE`.
5. **Project management lives on the fork.** Issues, milestones, and the roadmap
   are maintained in the cryo-ocd repository.
6. **Transparency.** Planning happens in public: `ROADMAP.md`, these ADRs, and
   GitHub issues/milestones.

## Consequences

- cryo-ocd operates as an independent (hard) fork with no upstream sync; there
  is no expectation of merging changes back to `paradigmxyz/cryo`.
- The fork carries an ongoing obligation to credit upstream contributors
  correctly; this is enforced through review.
- Should upstream maintenance ever resume, cryo-ocd remains a separate project;
  any collaboration would be renegotiated at that point.
- If the project matures to broad public usefulness, maintainer-specific
  infrastructure (see ADR-0004) is expected to be removed so others can run
  their own.
