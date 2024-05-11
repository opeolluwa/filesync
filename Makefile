install-toolchain:
	sh ./scripts/install-toolchain


#install the application dependencies 
install-deps:
	@echo "Installing dependencies"
	@cd desktop && yarn install
	@cd mobile && yarn install


# run the dev server 
run-desktop: 
	cd desktop && yarn tauri dev


# build the binary for the current OS
build:
	yarn tauri build

## run the android in expo
run-mobile: 
	cd mobile && yarn dev --host


# run the android in emulator 
android:
	cd mobile && yarn android


# run all apps
start:
	make run-mobile 
	make run-desktop
