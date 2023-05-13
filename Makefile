config:
	rustup target add wasm32-unknown-unknown
	npm install -g sass
	cargo install --locked trunk
	cargo install --locked wasm-bindgen-cli

serve:
	cargo build
	trunk serve
