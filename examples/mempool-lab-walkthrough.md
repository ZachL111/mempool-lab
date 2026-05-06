# Mempool Lab Walkthrough

This note is the quickest way to read the extra review model in `mempool-lab`.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | fee pressure | 178 | ship |
| stress | mempool churn | 159 | ship |
| edge | block fit | 185 | ship |
| recovery | replacement margin | 248 | ship |
| stale | fee pressure | 235 | ship |

Start with `recovery` and `stress`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

`recovery` is the optimistic case; use it to make sure the scoring path still rewards strong signal.
