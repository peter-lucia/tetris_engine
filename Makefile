.DEFAULT_GOAL := help
.SILENT: help
.PHONY: help build

VERSION := latest


help: ## Show a list of available commands
	grep "##.*" $(MAKEFILE_LIST) | grep -v ".*MAKEFILE_LIST.*" | sed -E "s/:.*##/:/g" | column -t -s :


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

stop: ## Stop the docker container
	docker stop rust-tetris || echo "Nothing to stop"

run: stop build ## Run the docker image
	docker container prune -f
	#docker run -p 9000:8080 --rm -v $$(pwd):/app rust-tetris:${VERSION}
	docker run -p 9000:8080 -p 9001:8000 -v $$(pwd)/tetris_frontend:/app/tetris_frontend --name rust-tetris --rm rust-tetris:${VERSION}

run-debug: stop build  ## Run the docker image but open a bash shell instead
	docker container prune -f
	docker run -it -p 9000:8080 -p 9001:8000-v $$(pwd)/tetris_frontend:/app/tetris_frontend --name rust-tetris --rm rust-tetris:${VERSION} bash

deploy-clean:  ## Deletes the deployment and load balancer
	kubectl delete deploy rust-tetris-deployment  || echo "Does not exist";
	kubectl delete service rust-tetris-load-balancer  || echo "Does not exist";

deploy-local: build deploy-clean ## Deploy rust-tetris to the local kubernetes cluster
	# Docs: https://kubernetes.io/docs/concepts/workloads/controllers/deployment/
	kubectl apply -f deploy.yaml
	kubectl apply -f load_balancer.yaml

view-deploy-local:  ## View the pods that were deployed locally
	kubectl get pods --show-labels
	kubectl describe services rust-tetris

