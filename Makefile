setup:
	git clone git@github.com:ewasm/scout.git

build:
	cargo build --manifest-path=packager/Cargo.toml --release
	cargo build --manifest-path=scout/Cargo.toml --release

scout: build
	cargo build --lib --release --no-default-features --features=scout --target wasm32-unknown-unknown
	chisel run --config chisel.toml
	packager/target/release/packager 256 2 1 --scout > scout/sheth.yaml
	cp target/wasm32-unknown-unknown/release/sheth.wasm scout/sheth.wasm	
	scout/target/release/phase2-scout scout/sheth.yaml
test: build
	cargo build --bin binsheth --release
	packager/target/release/packager 256 2 1 > blob
	-target/release/binsheth blob
	rm blob