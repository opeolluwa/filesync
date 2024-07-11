# FIleSync: WiFi File Sharing Application

FileSync is a file-sharing application, targeting Linux, Mac, Windows and Android platforms
![screenshot](./screenshots/filesync.png)

## Disclaimer

_⚠️ The application is still a work in progress; thus some features may be inconsistent. To proceed anyway, see the [release page](https://github.com/opeolluwa/filesync/releases)_

## Getting Started

### General Requirement

To compile the application or modify it locally, you need the following dependencies:

- [Node.js](https://nodejs.org) - A JavaScript Runtime environment
- [npm](https://npmjs.com) - A JavaScript package manager.
- [Rust v1.70 or greater](https://www.rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.

Once you have the dependencies installed, clone the project and install the required packages:

```sh
git clone https://github.com/opeolluwa/filesync.git
cd filesync 
npm run setup 
```

### To set up the Desktop Application

The project may require some system requirement based on your platform see the necessary 
[prerequisite](https://tauri.app/v1/guides/getting-started/prerequisites/) for your operating system.

### To Set up the mobile

_TBD_

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
