build:  # builds the test binary
	cargo build

cuke: build  # runs all end-to-end tests
	rm -rf tmp
	cargo test --test=cucumber

cukethis: build  # runs only end-to-end tests with a @this tag
	rm -rf tmp
	cargo test --test cucumber -- -t @this

fix:  # applies all auto-fixers
	dprint fmt

help:  # shows all available Make commands
	cat Makefile | grep '^[^ ]*:' | grep -v '.SILENT:' | grep -v help | sed 's/:.*#/#/' | column -s "#" -t

install:  # installs the binary on the current machine
	cargo install --path .

lint:  # finds code smells
	cargo clippy --all-targets --all-features -- -W clippy::pedantic -A clippy::cast_sign_loss -A clippy::cast_possible_truncation

run:  # runs in the local directory
	cargo run --quiet

test: unit cuke lint  # run all tests

unit:  # runs the unit tests
	cargo test

update:  # updates all dependencies
	cargo upgrade    # install cargo-edit if this doesn't work

.SILENT:
.DEFAULT_GOAL := help
