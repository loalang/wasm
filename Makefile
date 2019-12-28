.SILENT:
.PHONY: build version dist

VERSION ?= $(shell toml get ../loalang/Cargo.toml 'package.version' | jq -r)
ASSET_PATH=https://cdn.loalang.xyz/$(VERSION)/
BUCKET_PATH=gs://cdn.loalang.xyz/$(VERSION)/

build:
	rm -rf dist && ASSET_PATH=$(ASSET_PATH) yarn webpack --mode production

version:
	echo $(VERSION)

dist: build
	gsutil -m cp -r dist/* $(BUCKET_PATH)
	gsutil setmeta -h "Content-Type: application/wasm" $(BUCKET_PATH)*.wasm
	gsutil setmeta -h "Cache-Control: public, max-age=31536000" $(BUCKET_PATH)*
