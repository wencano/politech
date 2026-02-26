.PHONY: dev css trunk run build-release rsync

dev: css trunk run

css:
	npm run css:build

trunk: css
	trunk build index.html

run:
	cargo run --features server

# Release: CSS + WASM + release binary + bundle into target/release/
build:
	./scripts/build-release.sh

# Deploy: rsync to VM (default root@mcds.pro; override: make rsync DEPLOY_HOST=user@host)
rsync:
	./scripts/rsync.sh
