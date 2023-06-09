# Sendfile

Offline file-sharing application for Windows, Mac and Linux operating systems

![screenshot](screenshots/home-light.png)

## Table of Contents

- [Description](#description)
- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [License](#license)

## Description

Sendfile is a desktop application for Windows, Mac and Linux operating systems that allows you to send files seamlessly without the need for an internet connection.

## Getting Started

The following are required to run the application in development

- [Node.js](https://nodejs.org) - The JavaScript Runtime environment
- [Yarn](https://yarnpkg.com/) - A fast, secure and reliable package manager for Node.js
- [Rust v1.67 or greater](rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.

In addition, this project uses Tauri v1.3, see [prerequisite](https://tauri.app/v1/guides/getting-started/prerequisites/) for your operating system.

## Prerequisites

Sendfile is built on the following technologies

- [Tauri](https://tauri.app/) is a framework for building cross-platform desktop applications with front-end web technologies.
- [React](https://react.dev/) - JavaScript library for building user interfaces
- [Next.js](https://nextjs.org/) - React framework for the web
- [Rust](rust-lang.org/) - A language empowering everyone
  to build reliable and efficient software.
- [Typescript](https://typescript-lang.org) - A strongly-typed programming language that builds on JavaScript
- [TailwindCSS](https://tailwindcss.com) - A utility-first CSS framework

## Installation

To run the application in the development environment, set up the requirement specified in the [Getting Started](#getting-started) section afterward,

1. Install the UI dependencies, from the project root directory, run `yarn install`
2. Navigate to the application backend and install the dependencies, run `cd core && cargo run`
3. From the root directory, run

- `yarn tauri dev `  to run the application in development mode.
- `yarn tauri build `  to build the application executable

## License

This project is proprietary software owned by [Adeoye Adefemi](https://www.linkedin.com/in/adefemi-adeoye) and distributed under [MIT License](./LICENSE)
