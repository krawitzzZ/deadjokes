SHELL ?= /usr/bin/env bash
UID := $(shell id -u)
GID := $(shell id -g)

.PHONY: default
default: build

.PHONY: all-deps-graph
all-deps-graph:
	cargo depgraph --all-deps | dot -Tpng > all-deps-graph.png

.PHONY: ws-deps-graph
ws-deps-graph:
	cargo depgraph --workspace-only | dot -Tpng > ws-deps-graph.png

.PHONY: test
test:
	cargo test

.PHONY: build
build:
	cargo build

.PHONY: release
release:
	cargo build --release

.PHONY: image
image: release
	docker build -t deadjokes-api .

.PHONY: run
run:
	cargo run

.PHONY: watch
watch:
	cargo watch -x run

.PHONY: start
start: infra
	$(MAKE) run

.PHONY: elastic
elastic:
	docker compose -f ./docker-compose.elastic.yaml up -d

.PHONY: db
db:
	docker compose -f ./docker-compose.db.yaml up -d

.PHONY: infra
infra:
	docker compose -f ./docker-compose.elastic.yaml -f ./docker-compose.db.yaml up -d

.PHONY: up
up:
	docker compose -f ./docker-compose.elastic.yaml -f ./docker-compose.db.yaml -f ./docker-compose.api.yaml up -d

.PHONY: down
down:
	docker compose -f ./docker-compose.elastic.yaml -f ./docker-compose.db.yaml -f ./docker-compose.api.yaml down
