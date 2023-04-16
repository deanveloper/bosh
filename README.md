# bosh

Bosh is a linerider client.

## Installation

In the future I will provide prebuilt executables. Until then, here is how to install from source:

1. Install Tauri. Here is a [tutorial](https://tauri.app/v1/guides/getting-started/prerequisites/)
   which should insruct you on how to install Tauri on your system.
2. Clone the repository.
3. In the project root, run `npm install` to install dev dependencies.
4. Run `tauri dev` to run the development environment on your system.
    1. I don't recommend running `tauri build` as this builds a release build
       instead of a development build and takes a _much_ longer time.

### Architecture

* The physics engine is [bosh-rs]
* This project is a [bosh-rs] frontend using Tauri
* Vite and SolidJS are used for easy UI development

[bosh-rs]: https://github.com/deanveloper/bosh-rs