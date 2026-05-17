# Acknowledgements

cryo-ocd exists only because of the work that came before it.

## Original project

[`cryo`](https://github.com/paradigmxyz/cryo) was created and originally
maintained by **Paradigm**. cryo-ocd is a community-maintained fork that
continues that work — it is not affiliated with or endorsed by Paradigm. All of
the original architecture and design, and the overwhelming majority of the
code, are the work of the original authors.

## Upstream contributors

This fork incorporates bug fixes and features originally proposed by the cryo
community in the upstream issue and pull-request tracker. Every adopted change
is credited to its original author:

- in the commit history, via the `Author:` field (for cherry-picked commits) or
  a `Co-Authored-By:` trailer (for re-implemented changes);
- in [`CHANGELOG.md`](./CHANGELOG.md); and
- in the corresponding cryo-ocd issue, which links back to the upstream issue or
  pull-request number.

Contributors whose upstream work has been adopted are listed here as their
changes are merged. Issue and pull-request numbers below refer to the upstream
[`paradigmxyz/cryo`](https://github.com/paradigmxyz/cryo) tracker.

<!-- adopted-contributors:start -->
- [@0xhanh](https://github.com/0xhanh) — `--align` collection-summary
  divide-by-zero report (issue #125).
- [@0xMelkor](https://github.com/0xMelkor) — `raw` transaction column (PR #180).
- [@AuburyEssentian](https://github.com/AuburyEssentian) — reference patches for
  six Phase 2/3 correctness fixes (PRs #249, #251, #253, #254, #255, #256).
- [@BowTiedDevil](https://github.com/BowTiedDevil) — `--reorg-buffer`
  tail-chunk report (issue #193).
- [@ChadRosseau](https://github.com/ChadRosseau) — `erc20_transfers` and
  `erc721_transfers` fixes (issues #230, #231).
- [@Cybourgeoisie](https://github.com/Cybourgeoisie) — `--align`
  collection-summary divide-by-zero report (issue #150).
- [@Daulox92](https://github.com/Daulox92) — GitHub Actions supply-chain
  hardening (PR #242).
- [@distributedstatemachine](https://github.com/distributedstatemachine) —
  prebuilt release binaries and a `Dockerfile` (PR #40).
- [@dreaded369](https://github.com/dreaded369) — `default_sort`
  column-exclusion report (issue #221).
- [@dylantirandaz](https://github.com/dylantirandaz) — reference patch for the
  `--align` divide-by-zero fix (PR #258).
- [@Evan-Kim2028](https://github.com/Evan-Kim2028) — undeclared `cryo` Python
  package runtime dependencies report (issue #137).
- [@Hopium21](https://github.com/Hopium21) — documentation typo fixes
  (PR #226).
- [@LatentSpaceExplorer](https://github.com/LatentSpaceExplorer) — transaction
  `deploy_address` column (PR #189).
- [@mempirate](https://github.com/mempirate) — empty-result `FixedBytes`
  event-column report (issue #238).
- [@peyha](https://github.com/peyha) — EIP-4844 block fields (PR #181).
- [@phqb](https://github.com/phqb) — `transaction_count` column (PR #223);
  `geth_state_diffs` unchanged-field report (issue #245).
- [@PixelPil0t1](https://github.com/PixelPil0t1) — GitHub Actions supply-chain
  hardening (PR #241).
- [@shouc](https://github.com/shouc) — trace-gas `u32` truncation report
  (issue #173).
- [@solanaXpeter](https://github.com/solanaXpeter) — documentation typo fixes
  (PR #237).
- [@sslivkoff](https://github.com/sslivkoff) — `cryo`'s original author;
  address-filter fix (issue #97) and event-decoding groundwork (issue #56,
  PR #215) adopted post-fork.
- [@strmfos](https://github.com/strmfos) — `std::io::Error` modernization in
  `build.rs` (PR #234).
- [@sunxunle](https://github.com/sunxunle) — documentation typo fixes
  (PR #220).
<!-- adopted-contributors:end -->

## License

The original software and this fork are both distributed under the MIT and
Apache-2.0 licenses; see [`LICENSE-MIT`](./LICENSE-MIT) and
[`LICENSE-APACHE`](./LICENSE-APACHE).
