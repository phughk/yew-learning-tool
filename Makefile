config:
	rustup target add wasm32-unknown-unknown
	npm install -g sass
	cargo install --locked trunk
	cargo install --locked wasm-bindgen-cli
	npm i -g tailwindcss

serve:
	tailwindcss -o ./tailwind.css
	cargo build
	trunk serve

release:
	trunk build --release
