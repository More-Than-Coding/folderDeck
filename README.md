# Installation

1. Install dependencies: `yarn dev`

## Development Prerequisites

### Environment variables

The following are the environments used and will indicate if required for development. These need to be stored at a shell level such as in a `~/.zshrc` file.

- `APTABASE_ID` (optional): for analytics tracking

### Code Signing

There are several environment variables for [macOS Code Signing](https://tauri.app/v1/guides/distribution/sign-macos) that are not required for development. When building locally, you will create a version of the app that will be rejected by OS security if sent to another computed as it is not signed. The build however will work on your local device as it was created there.

### AutoUpdater

There are also environment variables used for auto-updating that are [defined here](https://tauri.app/v1/guides/distribution/updater). The secret for this will not be distributed for contributors due to security reasons.

### Github Action

You need to enable the github actions write authorization for the repository. This is needed to publish the releases and to create a branch for the updater.

## Tech stack

This template uses the following technologies:

- [Vue 3](https://v3.vuejs.org/)
- [Vite](https://vitejs.dev/)
- [Tauri](https://tauri.studio/)
- [Pinia](https://pinia.esm.dev/)
- [Vue Router](https://next.router.vuejs.org/)

It also includes the following features:

- Auto publish release when committing to the `main` branch
- Automatically support tauri's auto updater by generating and uploading the latest.json file to a separate branch
- Template comes preinstalled and preconfigured with Pinia and Vue Router

### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Version Management

There is a command line prompt that can be used to create new version branches or update the existing version within a branch. Simply use `yarn update-version` and follow the prompts.

## Update Cargo Dependencies

Simply run `yarn update-cargo` to update all cargo dependencies.
