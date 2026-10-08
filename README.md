# Quicknote

A lightweight, tabbed, auto-saving notepad for Linux (KDE Plasma / Wayland or X11),
written in Rust with [egui](https://egui.rs) / [eframe](https://github.com/emilk/eframe_template).

Unlike a full text editor, Quicknote stays out of the way: it's a borderless window
with a custom titlebar, designed to hold a few "todo" lists you glance at all day.

## Features

- **Collapsible titlebar** — the special `❐` button shrinks the window to just the
  titlebar (showing only the 4 buttons). Click `▢` to restore the previous size and
  position.
- **Custom 4-button titlebar** — `❐` collapse, `—` minimize, `□` maximize, `✕` close.
  Fully frameless (no native titlebar).
- **Draggable & resizable** — drag the title to move; drag the window edges to resize.
- **Tabs** — add, rename (double-click a tab), and close multiple named notes.
- **Configurable color** — set the whole-window background and text color, or pick a
  preset. Makes the collapsed titlebar easy to spot on your desktop.
- **Auto-save** — text, tab names, layout, and colors are saved to
  `~/.local/share/quicknote/state.json` (debounced ~500ms and on close), so a crash or
  fast reboot won't lose your notes.

## Build

```bash
cargo build --release
```

Binary: `target/release/quicknote`

## Run

```bash
./target/release/quicknote
```

## Autostart with KDE Plasma

```bash
mkdir -p ~/.config/autostart
# edit data/com.jcgit1.quicknote.desktop so Exec points to your release binary, then:
cp data/com.jcgit1.quicknote.desktop ~/.config/autostart/
```

## Usage

| Action | Result |
|--------|--------|
| Click a tab | Switch to it |
| Double-click a tab | Rename it |
| `×` on a tab | Close it |
| `＋` | New note |
| `❐` (titlebar) | Collapse to titlebar-only |
| `—` | OS minimize |
| `□` | Maximize / unmaximize |
| `✕` | Close (saves first) |
| `⚙` | Appearance (color) settings |
