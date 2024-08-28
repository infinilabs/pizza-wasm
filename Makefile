

build:
	wasm-pack build --dev

release:
	RUSTFLAGS="-Zlocation-detail=none" wasm-pack build --release \
        --manifest-path ./Cargo.toml \
        -Z build-std=panic_abort,std -Z build-std-features=panic_immediate_abort


publish:
	wasm-pack publish

# wasm-opt is a component of the Binaryen toolkit that optimizes WebAssembly modules.
optimize:
	wasm-opt -Oz   pkg/pizza_wasm_bg.wasm -o pkg/pizza_wasm_bg.wasm

# wasm-snip replaces a Wasm function's body with an unreachable instruction.
snip:
	wasm-snip pkg/pizza_wasm_bg.wasm -o pkg/pizza_wasm_bg.wasm

inspect:
	wasm-objdump -x pkg/pizza_wasm_bg.wasm

analysis:
	wasm-objdump -d \
    pkg/pizza_wasm_bg.wasm \
    | rustfilt | less

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

dist:
	(cd web && rm -f dist/*  && npm run build)

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

