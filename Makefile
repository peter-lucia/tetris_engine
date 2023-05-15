.DEFAULT_GOAL := help
.SILENT: help
.PHONY: help build


help: ## Show a list of available commands
	grep "##.*" $(MAKEFILE_LIST) | grep -v ".*MAKEFILE_LIST.*" | sed -E "s/:.*##/:/g" | column -t -s :


build-lib-darwin: ## Build the rust library for macOS
	cargo build --lib --target x86_64-apple-darwin --package tetris_engine_backend

build-bin-darwin: ## Build the rust binary app for macOS
	cargo build --bin tetris_engine_backend --target x86_64-apple-drawin --package tetris_engine_backend

test-rust: ## Run the rust unit tests
	cargo test --lib --package=tetris_engine_backend

test-python: ## Run the python unit tests
	python setup.py install
	python -m pytest --log-cli-level=DEBUG -s tests/