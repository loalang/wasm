.SILENT:

VERSION ?= $(shell toml get ../loalang/Cargo.toml 'package.version' | jq -r)
ASSET_PATH=https://cdn.loalang.xyz/$(VERSION)/

.PHONY: build
build: build-scripts build-libs

.PHONY: build-scripts
build-scripts:
	rm -rf dist && ASSET_PATH=$(ASSET_PATH) yarn webpack --mode production

.PHONY: build-libs
build-libs:
	cd loa && \
		rm -rf gen && \
		wasm-pack build --release --out-dir "gen" --scope "loalang" && \
		cd gen && \
		yarn version --no-git-tag-version --new-version $(VERSION) && \
		cp ../src/lib.d.ts ./loa.d.ts

.PHONY: version
version:
	echo $(VERSION)
