# FIleSync: WiFi File Sharing Application


FileSync is a file-sharing application, targeting Linux, Mac, Windows and Android platforms
![screenshot](./screenshots/filesync.png)


## Installation 
_⚠️ The application is still a work in progress; thus some features may missing. To proceed anyway,  see the [release page]()_

## Getting Started

To run the application in development, you'll need the following dependencies:

- [Node.js](https://nodejs.org) - A JavaScript Runtime environment
- [Yarn or](https://yarnpkg.com/) - A fast, secure and reliable package manager for Node.js
- [Rust v1.70 or greater](https://www.rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.

In addition, this project uses Tauri v1.3, see [prerequisite](https://tauri.app/v1/guides/getting-started/prerequisites/) for your operating system.

Once you have the dependencies installed, clone the project and install the required packages:

```sh
git clone https://github.com/opeolluwa/filesync.git
cd filesync # navigate to the cloned directory
yarn install # install the dependencies
yarn tauri dev # run the application locally

```

## Roadmap

- [x] Adaptive UI
- [x] build an executable for the Mac operating system
- [x] build an executable for the Linux Operating system
- [x] build an executable for Window
- [x] support mobile devices
- [ ] create a wifi hotspot on Unix Operating Systems 
- [ ] support dark mode
- [ ] add walkthrough after installation

## Acknowledgements

- [Linux Wifi Hotspot](https://awesomeopensource.com/project/elangosundar/awesome-README-templateshttps://github.com/lakinduakash/linux-wifi-hotspot)
- [File Streaming](https://github.com/tokio-rs/axum/tree/main/examples/stream-to-file)

## Contributing

Contributions are always welcome!

See [contributing.md](./CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [code of conduct](CODE_OF_CONDUCT.md).

## License

This project is distributed under the [MIT License](./LICENSE)
