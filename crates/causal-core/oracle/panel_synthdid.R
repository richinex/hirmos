# Golden fixture for long-panel preparation and the simultaneous-adoption
# DID / SC / SDID estimators. The reference source is vendored under
# reference/synthdid at commit 70c1ce3eac58e28c30b67435ca377bb48baa9b8a.
#
# Run with the pinned R container (no R packages are needed for point estimates):
# DOCKER_HOST=unix:///Users/richard/.colima/default/docker.sock docker run --rm \
#   -v "$PWD:/work" -w /work r-base:4.5.1 \
#   Rscript oracle/panel_synthdid.R

options(warn = 1)

source("reference/synthdid/R/solver.R")
source("reference/synthdid/R/utils.R")
source("reference/synthdid/R/synthdid.R")
source("reference/synthdid/R/vcov.R")

panel <- read.csv(
  "reference/synthdid/data/california_prop99.csv",
  sep = ";",
  stringsAsFactors = FALSE,
  check.names = FALSE
)
setup <- suppressWarnings(panel.matrices(panel))
noise.level <- sd(apply(setup$Y[1:setup$N0, 1:setup$T0], 1, diff))
did <- did_estimate(setup$Y, setup$N0, setup$T0)
sc <- sc_estimate(setup$Y, setup$N0, setup$T0)
sdid <- synthdid_estimate(setup$Y, setup$N0, setup$T0)

# Algorithm 4 placebo draws. Store the sampled control permutations as part of
# the numerical boundary so Rust is tested against the estimator rather than R's
# unrelated RNG implementation.
placebo.draws <- function(estimate, replications, seed) {
  setup <- attr(estimate, "setup")
  opts <- attr(estimate, "opts")
  weights <- attr(estimate, "weights")
  N1 <- nrow(setup$Y) - setup$N0
  if (setup$N0 <= N1) stop("placebo SE requires more controls than treated units")
  set.seed(seed)
  indices <- replicate(replications, sample(seq_len(setup$N0)), simplify = FALSE)
  estimates <- vapply(indices, function(ind) {
    placebo.N0 <- length(ind) - N1
    weights.boot <- weights
    weights.boot$omega <- sum_normalize(weights$omega[ind[seq_len(placebo.N0)]])
    as.numeric(do.call(synthdid_estimate, c(list(
      Y = setup$Y[ind, ], N0 = placebo.N0, T0 = setup$T0,
      X = setup$X[ind, , ], weights = weights.boot
    ), opts)))
  }, 0.0)
  standard.error <- sqrt((replications - 1) / replications) * sd(estimates)
  set.seed(seed)
  upstream.standard.error <- placebo_se(estimate, replications)
  stopifnot(abs(standard.error - upstream.standard.error) < 1e-14)
  list(indices = indices, estimates = estimates, standard.error = standard.error)
}

sc.placebos <- placebo.draws(sc, 24, 134)
sdid.placebos <- placebo.draws(sdid, 24, 134)
sc.in.time.error <- tryCatch({ synthdid_placebo(sc); NA_character_ }, error = conditionMessage)
# The pinned package's sc_estimate wrapper supplies omega.intercept twice when
# called by synthdid_placebo, so the public SC in-time helper errors. Evaluate
# its intended numerical call directly and retain the upstream error in the
# fixture rather than hiding it.
sc.placebo.T0 <- floor(setup$T0 * setup$T0 / ncol(setup$Y))
sc.opts <- attr(sc, "opts")
sc.in.time <- do.call(synthdid_estimate, c(list(
  Y = setup$Y[, seq_len(setup$T0)], N0 = setup$N0, T0 = sc.placebo.T0,
  weights = list(lambda = rep(0, sc.placebo.T0))
), sc.opts))
sdid.in.time <- synthdid_placebo(sdid)

# A second treated unit exercises the block-average path rather than only the
# single-treated-unit special case used in the published California analysis.
multi.panel <- panel
multi.panel$treated[multi.panel$State == "Kansas" & multi.panel$Year >= 1989] <- 1
multi.setup <- suppressWarnings(panel.matrices(multi.panel))
multi.did <- did_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)
multi.sc <- sc_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)
multi.sdid <- synthdid_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)
multi.sdid.placebos <- placebo.draws(multi.sdid, 12, 805)

json.number <- function(value) {
  if (!is.finite(value)) stop("fixture contains a non-finite number")
  trimws(formatC(value, digits = 17, format = "g"))
}
json.numbers <- function(values) {
  paste0("[", paste(vapply(as.numeric(values), json.number, ""), collapse = ","), "]")
}
json.strings <- function(values) {
  paste0("[", paste(vapply(as.character(values), function(value) encodeString(value, quote = '"'), ""), collapse = ","), "]")
}
json.matrix <- function(value) {
  rows <- vapply(seq_len(nrow(value)), function(row) json.numbers(value[row, ]), "")
  paste0("[", paste(rows, collapse = ","), "]")
}
nonnull.values <- function(values) {
  if (is.null(values)) numeric(0) else values[!is.na(values)]
}
estimate.json <- function(estimate) {
  weights <- attr(estimate, "weights")
  paste0(
    "{\"estimate\":", json.number(as.numeric(estimate)),
    ",\"lambda\":", json.numbers(weights$lambda),
    ",\"omega\":", json.numbers(weights$omega),
    ",\"effect_curve\":", json.numbers(synthdid_effect_curve(estimate)),
    ",\"lambda_values\":", json.numbers(nonnull.values(weights$lambda.vals)),
    ",\"omega_values\":", json.numbers(nonnull.values(weights$omega.vals)),
    "}"
  )
}
case.json <- function(setup, did, sc, sdid) {
  paste0(
    "{\"units\":", json.strings(rownames(setup$Y)),
    ",\"n0\":", setup$N0,
    ",\"t0\":", setup$T0,
    ",\"did\":", estimate.json(did),
    ",\"sc\":", estimate.json(sc),
    ",\"sdid\":", estimate.json(sdid),
    "}"
  )
}
placebo.json <- function(value) {
  paste0(
    "{\"indices\":[", paste(vapply(value$indices, json.numbers, ""), collapse = ","), "]",
    ",\"estimates\":", json.numbers(value$estimates),
    ",\"standard_error\":", json.number(value$standard.error),
    "}"
  )
}

json <- paste0(
  "{\"reference_commit\":\"70c1ce3eac58e28c30b67435ca377bb48baa9b8a\"",
  ",\"input\":{",
    "\"unit\":", json.strings(panel$State),
    ",\"time\":", json.numbers(panel$Year),
    ",\"outcome\":", json.numbers(panel$PacksPerCapita),
    ",\"treatment\":", json.numbers(panel$treated),
  "}",
  ",\"matrix\":{",
    "\"units\":", json.strings(rownames(setup$Y)),
    ",\"times\":", json.numbers(colnames(setup$Y)),
    ",\"n0\":", setup$N0,
    ",\"t0\":", setup$T0,
    ",\"noise_level\":", json.number(noise.level),
    ",\"y\":", json.matrix(setup$Y),
    ",\"w\":", json.matrix(setup$W),
  "}",
  ",\"did\":", estimate.json(did),
  ",\"sc\":", estimate.json(sc),
  ",\"sdid\":", estimate.json(sdid),
  ",\"inference\":{",
    "\"replications\":24,\"seed\":134",
    ",\"sc_placebo\":", placebo.json(sc.placebos),
    ",\"sdid_placebo\":", placebo.json(sdid.placebos),
    ",\"sc_in_time_upstream_error\":", encodeString(sc.in.time.error, quote = '"'),
    ",\"sc_in_time\":", estimate.json(sc.in.time),
    ",\"sdid_in_time\":", estimate.json(sdid.in.time),
    ",\"multi_sdid_placebo\":", placebo.json(multi.sdid.placebos),
  "}",
  ",\"multi_treated\":", case.json(multi.setup, multi.did, multi.sc, multi.sdid),
  "}"
)

writeLines(json, "oracle/fixtures/panel_synthdid.json", useBytes = TRUE)
cat(sprintf("California Prop 99: %d rows, %d controls, %d treated, T0=%d, T1=%d\n",
            nrow(panel), setup$N0, nrow(setup$Y) - setup$N0,
            setup$T0, ncol(setup$Y) - setup$T0))
cat(sprintf("  DID  %+.12f\n", as.numeric(did)))
cat(sprintf("  SC   %+.12f\n", as.numeric(sc)))
cat(sprintf("  SDID %+.12f\n", as.numeric(sdid)))
cat(sprintf("  SDID lambda iterations %d, omega iterations %d\n",
            sum(!is.na(attr(sdid, "weights")$lambda.vals)),
            sum(!is.na(attr(sdid, "weights")$omega.vals))))
cat(sprintf("  Placebo SE: SC %.12f, SDID %.12f (%d refits each)\n",
            sc.placebos$standard.error, sdid.placebos$standard.error,
            length(sc.placebos$estimates)))
cat(sprintf("  In-time placebo: SC %+.12f, SDID %+.12f\n",
            as.numeric(sc.in.time), as.numeric(sdid.in.time)))
