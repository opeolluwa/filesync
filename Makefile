install-toolchain:
	npm install -g yarn 
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#install the system depencies 
install-deps:
	yarn install
	cd core && cargo build
# run the dev server 
dev: 
	yarn tauri dev
# build the binary for the current OS
build:
	yarn tauri build