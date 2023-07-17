# Tauri audio application

Uses:

- [Tauri](https://tauri.app/)
- [Svelte](https://svelte.dev/)
- [Rust](https://www.rust-lang.org/)

Goal:

- play audio files in a cross-platform application
- audio files are retrieved from the backend

## Pre-requisites

- https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-macos
- https://tauri.app/v1/guides/building/macos

## Getting started

```bash
pnpm tauri dev
pnpm tauri build

pnpm tauri dev --target aarch64-apple-darwin
```

## Bugs

- I cannot hear the audio

### Attempts

```bash
pnpm tauri build
# or
pnpm tauri dev --target aarch64-apple-darwin
```

----

`src-tauri/tauri.conf.json`

Adding `tauri.bundle.appimage`: 

```json
"appimage":{
    "bundleMediaFramework": true
}
```

----

Having `tauri.allowlist`: 

```json
{
    "all": true,
    "shell": {
        "all": false,
        "open": true
    }
}
```

or

```json
{
    "all": true,
    "shell": {
        "all": false,
        "open": true
    },
    "protocol": {
        "asset": true,
        "assetScope": ["**"]
    }
}
```


### Setup

```bash
pnpm run tauri info
```

```
[✔] Environment
    - OS: Mac OS 13.4.1 X64
    ✔ Xcode Command Line Tools: installed
    ✔ rustc: 1.73.0-nightly (4c8bb79d9 2023-07-15)
    ✔ Cargo: 1.73.0-nightly (694a57956 2023-07-11)
    ✔ rustup: 1.26.0 (5af9b9484 2023-04-05)
    ✔ Rust toolchain: nightly-aarch64-apple-darwin (default)
    - node: 18.16.1
    - pnpm: 8.6.3
    - yarn: 1.22.19
    - npm: 9.5.1

[-] Packages
    - tauri [RUST]: 1.4.1
    - tauri-build [RUST]: 1.4.0
    - wry [RUST]: 0.24.3
    - tao [RUST]: 0.16.2
    - @tauri-apps/api [NPM]: 1.4.0
    - @tauri-apps/cli [NPM]: 1.4.0

[-] App
    - build-type: bundle
    - CSP: unset
    - distDir: ../dist
    - devPath: http://localhost:1420/
    - framework: Svelte
    - bundler: Vite
```
