# mempool-lab

`mempool-lab` explores crypto and simulation in Rust. The repository keeps the core rule set compact, then surrounds it with examples that show how the decisions move.

## Mempool Lab Notes

The quickest review path is the verifier first, then the fixtures, then the operations note. That order makes it easy to see whether the code, data, and explanation still agree.

## Why This Exists

The goal is to capture the core behavior in code and make the surrounding assumptions obvious. A reader should be able to run the verifier, open the fixtures, and understand why each decision was made.

## Implementation Notes

The core is a scoring model over demand, capacity, latency, risk, and weight. That keeps transaction pressure, fee pressure, and block selection in one explicit decision path. The threshold is 152, with risk penalty 5, latency penalty 3, and weight bonus 6. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Example Scenarios

`pressure` is the first example I would inspect because it lands on the `review` path with a score of 96. The broader file also keeps `degraded` at 5 and `surge` at 259, which gives the model a useful low-to-high spread.

## Feature Notes

- Uses fixture data to keep fee pressure changes visible in code review.
- Includes extended examples for block selection, including `surge` and `degraded`.
- Documents replacement policy tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.

## Local Setup

The only required setup is the local Rust toolchain. After cloning, stay in the repo root so fixture paths resolve correctly.

## Tests

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Code Tour

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Boundaries

The fixture set is deliberately small. That keeps the review surface clear, but it also means the model should not be treated as a complete domain simulator.

## Roadmap

- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add a short report command that prints the score breakdown for a single scenario.
- Add malformed input fixtures so the failure path is as visible as the happy path.
- Add one more crypto and simulation fixture that focuses on a malformed or borderline input.

## Try It

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.
