setup:
	git clone git@github.com:ewasm/scout.git

build:
	cargo build --release --no-default-features --features=scout --target wasm32-unknown-unknown 
	chisel run --config chisel.toml
	cargo build --manifest-path=packager/Cargo.toml --release
	cargo build --manifest-path=scout/Cargo.toml --release

test: build
	packager/target/release/packager > scout/sheth.yaml
	cp target/wasm32-unknown-unknown/release/sheth.wasm scout/sheth.wasm	
	scout/target/release/phase2-scout scout/sheth.yaml
