# Makefile

.PHONY: dev build export start tauri lint android desktop

dev:
	npm run dev --prefix app

build:
	npm run build --prefix app

export:
	npm run export --prefix app

start:
	npm run start --prefix app

tauri:
	npm run tauri --prefix app

lint:
	npm run lint --prefix app

android:
	source ~/.bash_profile && npm run android --prefix app

desktop:
	npm run desktop --prefix app
