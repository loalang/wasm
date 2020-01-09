.SILENT:
.PHONY: build build-scripts build-libs version dist dist-scripts dist-libs

VERSION ?= $(shell toml get ../loalang/Cargo.toml 'package.version' | jq -r)
ASSET_PATH=https://cdn.loalang.xyz/$(VERSION)/
BUCKET_PATH=gs://cdn.loalang.xyz/$(VERSION)/

build: build-scripts build-libs

build-scripts:
	rm -rf dist && ASSET_PATH=$(ASSET_PATH) yarn webpack --mode production

build-libs:
	cd loa && \
		rm -rf gen && \
		wasm-pack build --release --out-dir "gen" --scope "loalang" && \
		cd gen && \
		yarn version --no-git-tag-version --new-version $(VERSION) && \
		cp ../src/lib.d.ts ./loa.d.ts

version:
	echo $(VERSION)

dist: dist-scripts dist-libs

dist-scripts: build-scripts
	gsutil -m cp -r dist/* $(BUCKET_PATH)
	gsutil setmeta -h "Content-Type: application/wasm" $(BUCKET_PATH)*.wasm
	gsutil setmeta -h "Cache-Control: public, max-age=31536000" $(BUCKET_PATH)*

dist-libs: build-libs
	cd loa/gen && yarn publish --no-git-tag-version --new-version $(VERSION) --access public
