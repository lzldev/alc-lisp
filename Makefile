build-release:
	cargo build --profile=release --features=mimalloc

build-fast:
	cargo build --profile=fast --features=mimalloc
	

build-wasm:
	wasm-pack build ./alc-lisp-wasm --out-dir ./alc-lisp-wasm/pkg  --target web -v
	
docs-build:
	cargo doc --no-deps --workspace --document-private-items --open 