# Failure Modes

For `mempool-lab`, I would look first for these mistakes:

- `fee pressure` cases moving lanes without a matching threshold change.
- `block fit` scoring higher after drag increases.
- Duplicate fixture ids hiding a stale golden row.
- README examples drifting away from the verifier.

The local checks are intentionally strict about these cases.
