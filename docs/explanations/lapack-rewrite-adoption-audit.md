# LAPACK rewrite adoption audit

Status: audited and source-compatible adoption completed 8 September 2026

This document records where Hirmos can reuse the parity-tested LAPACK rewrites in
`rust-causal-transpile`, what the `enhance-numerical-recipes` branch has adopted, and where using
them would be the wrong substitution. A completed label means the transpile oracle test passes, the
same change has been moved into Hirmos, its focused Hirmos test passes where one exists, and the
whole causal-core crate compiles for WASM. It does not broaden the numerical operation beyond the
one used by the pinned source.

The six available numerical families are:

| Routine | Source-level operation it reproduces | Typical use |
| --- | --- | --- |
| DGELSD | `numpy.linalg.lstsq` / compatible SciPy least squares | Minimum-norm regression, including rank-deficient designs |
| DGESDD | `numpy.linalg.svd` and SVD-backed `pinv` | Singular values and vectors, pseudoinverses and numerical rank |
| DSYEVD | `numpy.linalg.eigh` / `eigvalsh` for real symmetric matrices | Covariance and kernel eigendecomposition |
| DGEEV | `numpy.linalg.eig` for real general matrices | Nonsymmetric eigenvalues and left/right eigenvectors, including complex pairs |
| DGETRF/DGETRS | Partial-pivot LU and a general linear solve | `solve`, inverse-by-solving and LU log determinants |
| DPOTRF/DPOTRS | Cholesky factorization and positive-definite solve | Covariance, information and kernel systems known to be positive definite |

The governing rule is source first: use a rewrite when the pinned Python, R or library source uses
the corresponding operation. Do not replace an algorithm merely because another decomposition is
mathematically capable of producing an answer.

## Implementation status

| Consumer | Oracle operation | Adopted recipe | Status on this branch |
| --- | --- | --- | --- |
| J-PCMCI+ ParCorrMult residualization | `numpy.linalg.lstsq` | DGELSD | Completed previously |
| ParCorrMult Wilks mode | SciPy inverse + NumPy `eigvalsh(UPLO='L')` | DGETRF/DGETRS + DSYEVD | Completed |
| KCI | NumPy `pinv` + `eigh` | DGESDD + DSYEVD | Completed |
| Shared statsmodels OLS | `OLS.fit(method='pinv')` | DGESDD | Completed |
| Back-door, front-door, HAC/WLS, stationarity, cointegration and VIF OLS callers | statsmodels OLS | Shared DGESDD OLS adapter | Completed |
| ARDL | `lstsq`, `pinv`, statsmodels OLS and `inv` | DGELSD, DGESDD and DGETRF/DGETRS | Completed |
| VECM lag selection | `lstsq` + positive-definite covariance log determinant | DGELSD + DPOTRF | Completed |
| VECM ML matrix square root and inverses | SciPy reduced SVD + NumPy `inv` | DGESDD + DGETRF/DGETRS | Completed |
| VECM ML eigenproblem | NumPy general `eig` | DGEEV + NumPy argsort ordering | Completed |
| oCSE log determinant | NumPy `slogdet` / partial-pivot LU | DGETRF | Completed |
| Causal Impact + LinearMediation | NumPy `pinv` | DGESDD | Completed |
| PC/FCI Fisher-Z | NumPy `inv` | DGETRF/DGETRS | Completed |
| ADF lag selection + Johansen preprocessing | statsmodels OLS / NumPy `pinv` + `inv` | DGESDD + DGETRF/DGETRS | Completed |
| Johansen generalized eigenproblem and normalization | NumPy `eig` + lower `cholesky` + `inv` | DGEEV + DPOTRF + DGETRF/DGETRS | Completed |
| Time-series diagnostics + VAR-LiNGAM | statsmodels OLS / `lstsq` + Cholesky | DGESDD + DGELSD + DPOTRF | Completed |
| Poisson GLM IRLS | `lstsq(rcond=-1)` then final WLS `pinv` | DGELSD + DGESDD | Completed |
| Poisson/negative-binomial optimizer systems | NumPy `solve` / `inv` | DGETRF/DGETRS | Completed |
| INGARCH intervention information | R `chol2inv(chol(...))` | upper DPOTRF/DPOTRS | Completed |
| IV2SLS + DYNOTEARS matrix exponential | NumPy `solve` | DGETRF/DGETRS | Completed |
| Linear-SCM counterfactual abduction | NumPy `solve` / `inv` | DGETRF/DGETRS | Completed |

The generated DGELSD, DGEEV, DGESDD and DSYEVD translations originally exported some identical internal
Fortran symbol names. That was harmless while they lived in separate experiments, but unsafe once
linked into one Hirmos crate and particularly unsafe beside Apple Accelerate. Their private support
symbols are now namespaced by driver. Public adapter calls remain ordinary Rust calls, and an
algorithm cannot accidentally bind to another driver's closure or to an ABI-incompatible system
symbol.

## What already uses DGELSD

Hirmos already contains the copied DGELSD implementation and a shared minimum-norm adapter in
[`least_squares.rs`](../../crates/causal-core/src/least_squares.rs). Three numerical paths call it
directly:

- [`parcorr.rs`](../../crates/causal-core/src/parcorr.rs) reproduces Tigramite
  `numpy.linalg.lstsq(..., rcond=None)`.
- [`parcorr_mult.rs`](../../crates/causal-core/src/parcorr_mult.rs) uses the same adapter for
  multivariate X/Y residualization.
- [`sklearn_linear.rs`](../../crates/causal-core/src/sklearn_linear.rs) centers predictors and the
  target before calling DGELSD with scikit-learn's relative cutoff.

These three callers already benefit more of Hirmos than their count suggests:

```text
DGELSD
├── ParCorr
│   ├── PCMCI
│   ├── PCMCI+
│   ├── LPCMCI
│   └── RPCMCI
├── ParCorrMult
│   └── J-PCMCI+
└── sklearn-compatible linear regression
    ├── CausalEffects
    ├── RPCMCI regime fitting
    ├── dynamic linear SCM fitting
    └── dynamic counterfactual fitting
```

For example, the J-PCMCI+ regression containing `incidents(t)`, `deploys(t-1)`, unit dummies and a
time-context variable already uses DGELSD. That part is complete. The separate Wilks-lambda option
now uses the two additional recipes described next.

## Completed: J-PCMCI+ ParCorrMult

The ordinary maximum-correlation lane was already covered by DGELSD. The optional
`PccaWilksLambda` lane in [`parcorr_mult.rs`](../../crates/causal-core/src/parcorr_mult.rs) formerly
used:

- faer's partial-pivot LU to invert `Cxx` and `Cyy`;
- nalgebra's symmetric eigendecomposition for the Wilks eigenvalues.

The pinned Tigramite source calls:

```python
M = scipy.linalg.inv(Cxx) @ Cxy @ scipy.linalg.inv(Cyy) @ Cxy.T
eigvals = numpy.linalg.eigvalsh(M)
```

The branch now uses DGETRF/DGETRS for the two general systems and DSYEVD for
`eigvalsh`. DSYEVD must consume the lower triangle because NumPy's default is `UPLO='L'`; Tigramite
does not symmetrize `M` by averaging its two triangles.

Benefit: all ParCorrMult correlation and significance modes share the same source-compatible
native/WASM numerical foundation, rather than only the default J-PCMCI+ path.

## Completed: KCI

[`kci.rs`](../../crates/causal-core/src/kci.rs) now uses the parity adapters for the two expensive
decompositions in conditional KCI:

1. the pseudoinverse of the regularized conditioning kernel;
2. the retained eigenfeatures of the residualized X and Y kernels.

The causal-learn source calls `numpy.linalg.pinv` and `numpy.linalg.eigh`. The direct mapping is:

- DGESDD for the pseudoinverse;
- DSYEVD for the symmetric eigenfeatures.

This affects FCI with KCI, PC with KCI, and graph-implication/falsification tests that select KCI.
It matters near two numerical boundaries: the pseudoinverse singular-value cutoff and the
`1e-5 * largest_eigenvalue` feature-retention threshold.

## Completed: one shared statsmodels OLS adapter

Several Hirmos modules said they reproduced statsmodels OLS but independently implemented their fit.
The pinned statsmodels default is `method="pinv"`: `pinv_extended` computes an SVD pseudoinverse,
the parameters, singular values, numerical rank and normalized covariance. It does not default to
QR.

The migration covered two former groups.

### Formerly using QR

- [`ols.rs`](../../crates/causal-core/src/ols.rs), which is consumed by ADF, KPSS,
  Zivot-Andrews and Engle-Granger cointegration.
- [`backdoor.rs`](../../crates/causal-core/src/backdoor.rs), including its placebo,
  random-common-cause and subset refuters.
- [`estimation.rs`](../../crates/causal-core/src/estimation.rs), for ordinary HAC OLS and WLS.

These fits worked on the existing full-rank fixtures, but a rank-deficient design could panic where
statsmodels returns a minimum-norm result and records the reduced rank.

### Formerly using an SVD through nalgebra

- [`frontdoor.rs`](../../crates/causal-core/src/frontdoor.rs), in both front-door stages.
- [`redundancy.rs`](../../crates/causal-core/src/redundancy.rs), where VIF intentionally encounters
  collinear designs.

These already expressed the correct pseudoinverse idea but did not use the parity-tested DGESDD
backend.

The shared [`ols.rs`](../../crates/causal-core/src/ols.rs) adapter now uses DGESDD once and returns a
coherent result containing:

- parameters and residuals;
- pseudoinverse and normalized covariance;
- singular values and numerical rank;
- residual degrees of freedom based on rank rather than the raw column count.

That avoids five subtly different OLS implementations while preserving the reference operation.
It benefits ordinary back-door estimation, two-stage front-door estimation, HAC/WLS inference,
stationarity and break tests, Engle-Granger cointegration and VIF.

## Completed: ARDL

[`ardl.rs`](../../crates/causal-core/src/ardl.rs) contains three different source operations. The
branch preserves them rather than flattening them into one generic "SVD solve":

| ARDL operation | Pinned statsmodels source | Rewrite |
| --- | --- | --- |
| Candidate-model residuals | `numpy.linalg.lstsq` | DGELSD |
| Partialling out the always-present deterministic block | `numpy.linalg.pinv` | DGESDD |
| Final OLS model and covariance | statsmodels OLS pseudoinverse path | shared DGESDD OLS adapter |

Lagged designs are often nearly collinear, so this migration improves both source parity and the
behavior of legitimate difficult inputs.

## Completed: VECM and Johansen general eigenproblems

[`vecm.rs`](../../crates/causal-core/src/vecm.rs) legitimately consumes almost the entire new
foundation:

- DGELSD for VAR lag-selection and coefficient least squares;
- DGESDD for statsmodels `_mat_sqrt`, which explicitly uses a reduced SVD;
- DGETRF/DGETRS for general systems currently expressed as matrix inverses;
- DPOTRF for the positive-definite residual-covariance log determinant.

Those four mappings are migrated. The maximum-likelihood eigenproblem now follows statsmodels'
fifth operation as well: DGEEV replaces the earlier symmetric-equivalent construction, its right
eigenvectors are reordered with the translated NumPy argsort semantics, and the VECM fixture still
passes.

Johansen now follows the pinned sequence directly: form `inv(skk) @ sig`, call DGEEV, normalize the
right eigenvectors through `inv(cholesky(du.T @ skk @ du))`, sort the roots descending, and apply
statsmodels' first-nonzero sign convention. The lower Cholesky factor comes from DPOTRF and its
inverse comes from DGETRF/DGETRS. The earlier Cholesky-whitened symmetric reformulation has been
removed.

## Completed: oCSE

[`ocse.rs`](../../crates/causal-core/src/ocse.rs) previously had two implementations of the LU
factorization behind `numpy.linalg.slogdet`:

- Apple Accelerate DGETRF on macOS;
- a handwritten recursive partial-pivot LU on other targets, including WASM.

The translated DGETRF now replaces both. This removes a duplicated numerical algorithm and gives
native and WASM the same rounding and pivot path.

## Completed second adoption tier

These source-backed sites affect smaller or less failure-prone matrices than the first wave, but
they now use the same verified adapters.

### Causal Impact

[`causal_impact.rs`](../../crates/causal-core/src/causal_impact.rs) initializes regression
coefficients with `numpy.linalg.pinv(exog).dot(residual)`. It now uses DGESDD with NumPy's default
relative cutoff.

### Tigramite LinearMediation

[`linear_mediation.rs`](../../crates/causal-core/src/linear_mediation.rs) constructs the lag-zero
response with `pinv(I - Phi[0])`. It now uses DGESDD, directly covering lagged total-effect and
dynamic mediation responses.

### PC/FCI Fisher-Z

[`constraint_discovery.rs`](../../crates/causal-core/src/constraint_discovery.rs) obtains a
precision submatrix with the DGETRF/DGETRS general inverse used by causal-learn's Fisher-Z path.
Singular correlation matrices continue to produce the existing typed refusal.

### Time-series diagnostics

[`tsdiag.rs`](../../crates/causal-core/src/tsdiag.rs) now preserves its separable source operations:

- PACF Yule-Walker systems: DGETRF/DGETRS;
- AutoReg's statsmodels OLS: shared DGESDD OLS adapter;
- statsmodels VAR fit with `numpy.linalg.lstsq(..., rcond=1e-15)`: DGELSD;
- positive-definite VAR residual-covariance log determinant: DPOTRF.

### VAR-LiNGAM

[`var_lingam.rs`](../../crates/causal-core/src/var_lingam.rs) uses DGELSD for the statsmodels VAR
coefficients and DPOTRF for its positive-definite covariance log determinant. Its adaptive-lasso
helper reuses the centered scikit-learn DGELSD adapter.

### Count estimators

[`glm.rs`](../../crates/causal-core/src/glm.rs) was traced through statsmodels rather than treated as
one generic weighted regression. Each IRLS iteration uses DGELSD with the source's `rcond=-1`; the
final WLS covariance uses DGESDD `pinv`; Poisson Newton steps use DGETRF/DGETRS `solve`; and the
negative-binomial covariance uses the same general inverse as statsmodels.

[`ingarch.rs`](../../crates/causal-core/src/ingarch.rs) follows tscount's upper-triangular
`chol2inv(chol(information))` with upper DPOTRF followed by DPOTRS against the identity.

### Smaller general systems

- [`iv.rs`](../../crates/causal-core/src/iv.rs): multi-instrument IV2SLS now uses DGETRF/DGETRS for
  both NumPy general solves.
- [`expm.rs`](../../crates/causal-core/src/expm.rs): the Padé numerator/denominator system used by
  DYNOTEARS now uses DGETRF/DGETRS.
- [`counterfactual.rs`](../../crates/causal-core/src/counterfactual.rs): exact and noisy linear-SCM
  abduction now use DGETRF/DGETRS for the oracle's NumPy solve/inverse systems.

### ADF and Johansen preprocessing

ADF automatic lag selection now fits each prefix through the same statsmodels pseudoinverse OLS
path as `_autolag`; the earlier single-QR shortcut was algebraically efficient but not the source's
operation. Johansen detrending and residualization now use DGESDD for the source's OLS/pseudoinverse
steps, and its `S00` inverse uses DGETRF/DGETRS. DGEEV then covers the source's later general
eigenproblem and right-eigenvector output.

## Boundaries and exclusions

### CD-NOTS and GRACE are `f32`

The causal-ts preparation and neural paths intentionally reproduce PyTorch `float32` arithmetic.
The new rewrites are double-precision routines. Using them would change the source semantics.
Exact adoption there requires the corresponding SGELSD, SSYEVD, SGETRF/SGETRS and SPOTRF/SPOTRS
routines.

### Synthetic control has a different solver contract

[`synthetic_control.rs`](../../crates/causal-core/src/synthetic_control.rs) intentionally uses a
custom active-set optimizer rather than the cvxpy solver in the reference. DGESDD could make its
internal KKT solve more robust, but that would not establish cvxpy solver parity.

### The HP filter is sparse in the source

Statsmodels' HP filter uses a sparse linear solver. A dense DPOTRF replacement may be valid
mathematically but would not reproduce that source path.

### LARS has specialized updates

The LARS core uses incremental Cholesky updates and downdates. Those are part of the algorithm and
should not be replaced by a full dense factorization. Only its separate scikit-learn linear
regressions are candidates for the existing DGELSD adapter.

### Generated internals are not shared public APIs

DGEEV, DGESDD and DSYEVD contain internal QR, triangular and BLAS helpers. Other Hirmos algorithms should
not reach into those generated modules. If Hirmos needs source-compatible QR or triangular solves,
port and expose the corresponding LAPACK driver deliberately rather than coupling a method to an
implementation detail of another driver.

## Failure modeling at the WASM boundary

The LAPACK wrappers report illegal dimensions, singular systems and convergence failures. Adopting
callers must preserve those failures as data.

Several current numerical functions call `.expect(...)` after a factorization. A rank-deficient or
singular dataset can therefore become a Rust panic and then a generic WASM worker failure. During
migration each public method should instead map the LAPACK result into its own error variant, for
example:

```text
DGESDD did not converge
        │
        ▼
KciError::EigenDecompositionFailure
        │
        ▼
analysis worker's typed method refusal
        │
        ▼
an actionable message in the selected method panel
```

No new successful-result shape is required merely because the backend changed. The TypeScript/WASM
protocol only needs expansion if a method gains a new failure variant that the user can act on. Any
such union must remain exhaustively interpreted at the worker and UI boundaries.

## Verification performed

The branch was verified at three levels:

1. The translated DGELSD, DGEEV, DGESDD, DSYEVD, DGETRF/DGETRS and DPOTRF/DPOTRS drivers passed their
   source-oracle fixtures in `rust-causal-transpile`.
2. Focused Hirmos tests passed for the migrated consumers, including statsmodels OLS,
   ParCorrMult, KCI, ARDL, VECM, oCSE, Causal Impact, LinearMediation, PC/FCI, time-series
   diagnostics, VAR-LiNGAM, GLM, INGARCH, IV, DYNOTEARS and linear-SCM counterfactuals.
3. The complete `hirmos-causal-core` test suite, including its compile-fail documentation tests,
   passed after the migrations. The complete crate also passed
   `cargo check --target wasm32-unknown-unknown`.

The test evidence establishes source parity for the operations and fixtures listed above. It does
not turn a deliberately excluded operation into a covered one: the `f32` causal-ts paths still
need the corresponding single-precision drivers.

## Deliberately separate follow-ups

1. Add runtime WASM numerical comparisons for completed consumers, not only raw driver tests; the
   entire Hirmos causal-core crate compiles for `wasm32-unknown-unknown` on this branch.
2. Add source-backed rank-deficient and near-singular cases where a consumer's existing fixture is
   currently full rank.

For every migration, keep the current oracle fixture, add rank-deficient and near-singular cases,
compare native and WASM results, and only remove the previous implementation after both paths agree
with the pinned source.
