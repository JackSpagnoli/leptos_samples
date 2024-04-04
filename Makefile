install: install-hooks install-rust-toolchains install-trunk install-cargo-dev install-node

install-build: install-rust-toolchains install-trunk install-node

install-rust-toolchains:
	rustup toolchain install nightly
	rustup override set nightly
	rustup target add wasm32-unknown-unknown
	rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu

install-trunk:
	cargo install trunk

install-cargo:
	cargo install leptosfmt

install-node:
	npm install

install-hooks:
	pip install pre-commit
	pre-commit install --install-hooks --overwrite

clean:
	rm -rf **/target
	rm -rf **/style
	rm -rf **/dist
	rm -rf **/node_modules
	rm -f **/Cargo.lock
