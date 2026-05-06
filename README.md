# mempool-lab

`mempool-lab` keeps a focused Rust implementation around crypto and simulation. The project goal is to simulate mempool admission, replacement rules, fee bands, and block selection.

## Why I Keep It Small

The project exists to keep a narrow engineering decision visible and testable. For this repo, that decision is how fee pressure and block fit should influence a review result.

## Mempool Lab Review Notes

The first comparison I would make is `replacement margin` against `mempool churn` because it shows where the rule is most opinionated.

## Included Behavior

- `fixtures/domain_review.csv` adds cases for fee pressure and mempool churn.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/mempool-lab-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `replacement margin` and `mempool churn`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Internal Model

The fixture data drives the tests. The code stays thin, while `metadata/domain-review.json` and `config/review-profile.json` explain what each case is meant to protect.

The Rust addition stays small enough to inspect in one sitting.

## Try It Locally

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Validation

The same command runs the local verification path. The highest-scoring domain case is `recovery` at 248, which lands in `ship`. The most cautious case is `stress` at 159, which lands in `ship`.

## Scope

No external service is required. A deeper version would add more negative cases and a clearer boundary around invalid input.
