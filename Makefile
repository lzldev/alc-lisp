build-release:
	cargo build --profile=release --features=mimalloc

build-fast:
	cargo build --profile=fast --features=mimalloc
	
build-fast-glibc:
	cargo build --profile=fast

build-wasm:
	wasm-pack build ./alc-lisp-wasm --out-dir ./alc-lisp-wasm/pkg  --target web -v
	
bench-release:
	cargo bench --profile=release 

bench-release-mimalloc:
	cargo bench --profile=release --features="mimalloc"

bench-fast-mimalloc:
	cargo bench --profile=fast --features="mimalloc"

bench-fast:
	cargo bench --profile=fast 
	
docs-build:
	cargo doc --no-deps --workspace --document-private-items --open 