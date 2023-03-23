#!bin/bash
# Build the UI
yarn build 
#copy the ui files to the server
yarn export
#copy the splash screen
cp -r public/splashscreen.html out/