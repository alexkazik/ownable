#!/bin/bash

#
# Perform a few simple checks ahead of a PR
#

# Usage: `./check.sh` or `./check.sh <toolchain>`
# If the toolchain is omitted `+nightly` is used, `+stable` or `+beta` are the most common alternatives

TOOLCHAIN=${1:-+nightly}
echo Using toolchain $TOOLCHAIN

# builds
cargo $TOOLCHAIN build --release --all-features --tests || exit 1
cargo $TOOLCHAIN build --release --no-default-features --tests || exit 1

# clippy
cargo $TOOLCHAIN clippy --release --all-features --tests -- -D warnings || exit 1
cargo $TOOLCHAIN clippy --release --no-default-features --tests -- -D warnings || exit 1

# update formatting
cargo $TOOLCHAIN fmt --all || exit 1

# update readme
cargo rdme --force || exit 1

# create docs
if test "$TOOLCHAIN" = "+nightly"
then
  RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p ownable --all-features || exit 1
else
  echo "Skipping 'cargo doc' with doc_cfg since it's only available on nightly"
fi

# tests
cargo $TOOLCHAIN test --release --all-features -- --include-ignored || exit 1
