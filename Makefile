build:
	wasm-pack build --release

optimize:
	npm install -g binaryen
	wasm-opt -Oz -all --dce pkg/pizza_wasm_bg.wasm -o pkg/pizza-optimized.wasm
	cargo install wasm-snip
	wasm-snip pkg/pizza_wasm_bg.wasm -o pkg/pizza-snipped.wasm

inspect:
	#npm install -g @webassemblyjs/wabt
	brew install wabt
	wasm-objdump -x pkg/pizza_wasm_bg.wasm

bloat:
	cargo install cargo-bloat
	cargo bloat --release --target wasm32-unknown-unknown

twiggy:
	cargo install twiggy
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

# You can add additional phony targets as needed
.PHONY: build