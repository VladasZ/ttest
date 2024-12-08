#!/bin/bash
set -eox pipefail

rustup component add clippy

cargo clippy \
    -- \
    \
    -W clippy::all \
    -W clippy::pedantic \
    \
    -A clippy::must-use-candidate \
    -A clippy::return-self-not-must-use \
    -A clippy::missing-errors-doc \
    -A clippy::missing-panics-doc \
    -A clippy::needless-pass-by-value \
    -A clippy::module-name-repetitions \
    -A clippy::module_inception \
    \
    -D warnings
