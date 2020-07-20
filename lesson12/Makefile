restart: build purge run
rerun: purge run

expand:
	BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template > expanded-node.rs
    BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template-runtime > expanded-runtime.rs

build-run: build run

run:
	-pkill node-template
	target/debug/node-template --dev -d target/substrate --execution=Native --ws-external -lruntime=debug

benchmarks:
	cd node;cargo build --release --features runtime-benchmarks
	target/release/node-template benchmark \
		--chain=dev \
		--execution=wasm \
		--wasm-execution=compiled \
		--pallet=pallet-template \
		--extrinsic=do_something \
		--steps=20 \
		--repeat=51

purge:
	target/debug/node-template purge-chain --dev -y -d target/substrate

build:
	WASM_BUILD_TYPE=release cargo build

check:
	SKIP_WASM_BUILD= cargo check
check-wasm:
	WASM_BUILD_TYPE=release cargo check

check-debug:
	RUSTFLAGS="-Z external-macro-backtrace" BUILD_DUMMY_WASM_BINARY= cargo +nightly check

check-dummy:
	BUILD_DUMMY_WASM_BINARY= cargo check

