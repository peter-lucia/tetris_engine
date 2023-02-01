.DEFAULT_GOAL := help
.SILENT: help
.PHONY: help


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