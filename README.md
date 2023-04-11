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

## Example

file: agenda.org

```org
* My personal todo's
** DONE [#A] take out the trash
   SCHEDULED: <2023-04-07 Fri>
** TODO write a letter
   DEADLINE: <2023-04-09 Sun>
* My hobbies
** TODO [#A] learn rust
*** TODO [#B] learn Tauri
*** TODO [#C] learn Yew
*** TODO [#A] learn Leptos
** TODO buy a new skateboard
   DEADLINE: <2023-04-14 Fri>
```

this file would be shown like this:

### All Files View

![all files](https://github.com/jobnbackpack/org-mode-tauri/blob/main/public/Screenshot%202.png?raw=true)

### Today View

![today](https://github.com/jobnbackpack/org-mode-tauri/blob/main/public/Screenshot%201.png?raw=true)

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
- [ ] keybindings
