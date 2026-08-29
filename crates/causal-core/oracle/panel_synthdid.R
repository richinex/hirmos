# Golden fixture for long-panel preparation and the simultaneous-adoption
# DID / SC / SDID estimators. The reference source is not vendored here: it is read from the
# transpile checkout at ../../octopus/rust-causal-transpile/reference/synthdid (commit
# 70c1ce3eac58e28c30b67435ca377bb48baa9b8a), so regenerating the fixture needs that checkout beside this repository.
#
# Run with the pinned R container (no R packages are needed for point estimates):
# DOCKER_HOST=unix:///Users/richard/.colima/default/docker.sock docker run --rm \
#   -v "$PWD:/work" -w /work r-base:4.5.1 \
#   Rscript oracle/panel_synthdid.R

options(warn = 1)

source("../../octopus/rust-causal-transpile/reference/synthdid/R/solver.R")
source("../../octopus/rust-causal-transpile/reference/synthdid/R/utils.R")
source("../../octopus/rust-causal-transpile/reference/synthdid/R/synthdid.R")

panel <- read.csv(
  "../../octopus/rust-causal-transpile/reference/synthdid/data/california_prop99.csv",
  sep = ";",
  stringsAsFactors = FALSE,
  check.names = FALSE
)
setup <- suppressWarnings(panel.matrices(panel))
noise.level <- sd(apply(setup$Y[1:setup$N0, 1:setup$T0], 1, diff))
did <- did_estimate(setup$Y, setup$N0, setup$T0)
sc <- sc_estimate(setup$Y, setup$N0, setup$T0)
sdid <- synthdid_estimate(setup$Y, setup$N0, setup$T0)

# A second treated unit exercises the block-average path rather than only the
# single-treated-unit special case used in the published California analysis.
multi.panel <- panel
multi.panel$treated[multi.panel$State == "Kansas" & multi.panel$Year >= 1989] <- 1
multi.setup <- suppressWarnings(panel.matrices(multi.panel))
multi.did <- did_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)
multi.sc <- sc_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)
multi.sdid <- synthdid_estimate(multi.setup$Y, multi.setup$N0, multi.setup$T0)

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

