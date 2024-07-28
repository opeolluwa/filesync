# Makefile

.PHONY: dev build export start tauri lint android desktop

dev:
	npm run dev 

build:
	npm run build 

export:
	npm run export 

start:
	npm run start 

tauri:
	npm run tauri 

lint:
	npm run lint 

android:
	source ~/.bash_profile && npm run android 

desktop:
	npm run desktop 

build-apps:
	npm run tauri build 

# Makefile
