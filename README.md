# Send File
Seamless Share: Empowering Offline PC-to-PC File Transfer

_⚠️ Some features are missing since the application is still in development, see the [issues](https://github.com/opeolluwa/send-file/issues) page to contribute or leave a star_

![screenshot](screenshots/home-light.png)

## Table of Contents

- [Description](#description)
- [Getting Started](#getting-started)
- [Tech Stack](#technology-stack)
- [Features](#features)
- [Acknowledgement](#acknowledgements)
- [Contributing](#contributing)
- [License](#license)

## Description

An offline seamless file-sharing application for Windows, Mac, and Linux operating system

## Getting Started

The following are required to run the application in development

- [Node.js](https://nodejs.org) - The JavaScript Runtime environment
- [Yarn](https://yarnpkg.com/) - A fast, secure and reliable package manager for Node.js
- [Rust v1.67 or greater](rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.

In addition, this project uses Tauri v1.3, see [prerequisite](https://tauri.app/v1/guides/getting-started/prerequisites/) for your operating system.

Once the dependencies have been met, clone the project and install the dependencies

```sh
git clone https://github.com/opeolluwa/Send-file.git sendfile
cd sendfile # navigate to the cloned directory
yarn install # install the dependencies
yarn tauri dev # run the application locally

```

## Technology Stack

Sendfile is built primarily on the following technologies

- [Tauri](https://tauri.app/) is a framework for building cross-platform desktop applications with front-end web technologies.
- [React](https://react.dev/) - JavaScript library for building user interfaces
- [Next.js](https://nextjs.org/) - React framework for the web
- [Rust](rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.
- [Typescript](https://typescript-lang.org) - A strongly-typed programming language that builds on JavaScript

- [TailwindCSS](https://tailwindcss.com) - A utility-first CSS framework

## Features

- [x] Adaptive UI
- [x] scan to connect mobile
- [ ] create a wifi hotspot
- [ ] build the user interface for mobile devices
- [ ] support dark mode
- [ ] add walkthrough after installation
- [ ] build an executable for Window
- [x] build an executable for the Mac operating system
- [x] build an executable for the Linux Operating system

## Acknowledgements

- [Linux Wifi Hotspot](https://awesomeopensource.com/project/elangosundar/awesome-README-templateshttps://github.com/lakinduakash/linux-wifi-hotspot)
- [File Streaming](https://github.com/tokio-rs/axum/tree/main/examples/stream-to-file)
## Contributing

Contributions are always welcome!

See [contributing.md](./CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [code of conduct](CODE_OF_CONDUCT.md).

## License

This project is proprietary software owned by [Adeoye Adefemi](https://www.linkedin.com/in/adefemi-adeoye) and distributed under [MIT License](./LICENSE)
