
lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::must_use_candidate \
      \
      -D warnings

test:
	cargo test --all && cargo test --all --release
