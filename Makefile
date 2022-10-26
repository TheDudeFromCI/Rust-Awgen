default: build

build: clean docs
	@echo Building project binary
	@cargo build --release

test:
	@echo Testing all project files
	@cargo test --all-targets --features bevy/dynamic

run: test
	@echo Compiling development build
	@cargo run --features bevy/dynamic

fmt:
	@echo Validating project file formating
	@cargo fmt --all -- --check
	@cargo clippy --all-targets --all-features -- -D warnings

docs:
	@echo Building project documentation
	@cargo doc --document-private-items

clean:
	@echo Cleaning binary cache
	@rm -rf target

validate: test fmt docs
