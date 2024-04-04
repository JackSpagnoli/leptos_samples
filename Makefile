install: install-hooks install-rust-toolchains install-leptos install-tailwind

install-rust-toolchains:
	rustup toolchain install nightly
	rustup override set nightly
	rustup component add rustfmt --toolchain nightly-x86_64-unknown-linux-gnu

install-leptos:
	cargo install leptosfmt

install-tailwind:
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
