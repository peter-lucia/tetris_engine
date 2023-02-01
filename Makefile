.DEFAULT_GOAL := help
.SILENT: help
.PHONY: help build

VERSION := latest


help: ## Show a list of available commands
	grep "##.*" $(MAKEFILE_LIST) | grep -v ".*MAKEFILE_LIST.*" | sed -E "s/##//g" | column -t -s :


update: ## Ensures the submodules have the latest available changes
	git submodule foreach --recursive git checkout main
	git submodule foreach --recursive git pull origin main

clean: ## Puts the current repo and its submodules back to their 'main' branch
	git reset --hard
	git pull origin main
	git submodule foreach --recursive git reset --hard
	git submodule foreach --recursive git checkout main
	git submodule foreach --recursive git pull origin main

build: ## Build the docker image
	docker build . -f Dockerfile -t rust-tetris:${VERSION} --pull

run: build ## Run the docker image
	docker container prune -f
	docker run -p 9000:80 --rm -v $$(pwd):/app rust-tetris:${VERSION}

run-debug: build  ## Run the docker image but open a bash shell instead
	docker container prune -f
	docker run -it -p 9000:80 --rm  $$(pwd):/app rust-tetris:${VERSION} bash
