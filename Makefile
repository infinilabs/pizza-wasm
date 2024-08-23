

build:
	wasm-pack build --release

optimize:
	wasm-opt -Oz -all --dce pkg/pizza_wasm_bg.wasm -o pkg/pizza-optimized.wasm
	wasm-snip pkg/pizza_wasm_bg.wasm -o pkg/pizza-snipped.wasm

inspect:
	wasm-objdump -x pkg/pizza_wasm_bg.wasm

bloat:
	cargo bloat --release --target wasm32-unknown-unknown

twiggy:
	twiggy top pkg/pizza_wasm_bg.wasm
	twiggy dominators pkg/pizza_wasm_bg.wasm

# Compress the WASM binary with gzip
gzip:
	gzip -9 pkg/pizza_wasm_bg.wasm -c > pkg/pizza_wasm_bg.wasm.gz

fmt:
	cargo fmt

# Clean the build artifacts
clean:
	cargo clean
	rm -f pkg/*

serve:
	(cd web && npm run start)

# Init dev depends
init:
	cargo install wasm-pack
	(cd web && npm install)
	npm install -g binaryen
	cargo install wasm-snip
	cargo install cargo-bloat
	cargo install twiggy
	#mac only
	brew install wabt

# You can add additional phony targets as needed
.PHONY: build