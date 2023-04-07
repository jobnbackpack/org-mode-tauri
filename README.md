# Tauri (Rust + Svelte)

- Rust -> parse orgfiles and provide some api endpoints for the frontend
- Svelte -> Frontend renders my [org](https://orgmode.org/) files and provides
  different views to overview and manage my todos

## Current state

- in development!
- to parse your files you need to change the paths in main.rs
- right now I only parse and view data - org files are not manipulated yet
- two views are provided now:
  - 'All Files View' - show all org files of an directory with parsed headlines
    and todos
  - 'Today View' - show all todos scheduled/deadlined for today. Also show todos
    within a deadline range of 14 days

## Installing

- clone this repo
- install [pnpm](https://pnpm.io/installation)
- run `pnpm install`
- change paths in `src-tauri/src/main.rs`
- **for development** run `pnpm tauri dev`
- **for executable** run `pnpm tauri build` and run .exe/.dmg/.app

## Roadmap

- [ ] navigate through days
- [ ] correct view for repeating tasks
- [ ] cleanup code -> put most of the logic in rust
- [ ] weekly view
- [ ] settings - set folder path and deadline range for day view
- [ ] manipulate and write org files
  - actions like: date changes, new todos, finish todos,...
