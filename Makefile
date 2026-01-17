.DEFAULT_GOAL := help
.SILENT: help
.PHONY: help build


help: ## Show a list of available commands
	grep "##.*" $(MAKEFILE_LIST) | grep -v ".*MAKEFILE_LIST.*" | sed -E "s/:.*##/:/g" | column -t -s :


build-lib-darwin: ## Build the rust library for macOS
	cargo build --lib --target aarch64-apple-darwin --package tetris_engine_backend

rustup-list-targets: ## List available rustup targets
	rustup target list
	# rustup target add <specific target from list>

build-python-linux: ## Build the python package for a particular platform
	# https://cibuildwheel.readthedocs.io/en/stable/setup/
	cibuildwheel --platform linux

build-bin-darwin: ## Build the rust binary app for macOS
	cargo build --bin tetris_engine_backend --target aarch64-apple-darwin --package tetris_engine_backend

test-rust: ## Run the rust unit tests
	cargo test --lib --package=tetris_engine_backend

test: ## Run the python unit tests
	export PYTHONPATH=$$(pwd) && pytest tests/ --log-cli-level=DEBUG -s

bumpversion: ## Increment the package version
	# pip install -r requirements.txt -r requirements-test.txt
	read -p "Enter the new version: " new_version && \
		uv run bumpversion --new-version $$new_version --allow-dirty --no-commit --tag \
		pyproject.toml \
		tetris_engine/__init__.py \
		.bumpversion.cfg

setup: ## Setup the uv environment
	uv init --python 3.12
	source .venv/bin/activate

sync: ## Sync the uv environment
	uv sync --all-groups

build: ## build the target to dist/ as a wheel
	uv build --wheel

generate-ci: ## Generate the GitHub CI workflow file
	uv run maturin generate-ci github > .github/workflows/CI.yml