# Web-based GUI

{{#include web_based_gui.incl.md}}

## `tauri` {#tauri}

[![tauri-website][c-tauri-website-badge]][c-tauri-website] [![tauri][c-tauri-badge]][c-tauri] [![tauri-crates.io][c-tauri-crates.io-badge]][c-tauri-crates.io] [![tauri-github][c-tauri-github-badge]][c-tauri-github] [![tauri-lib.rs][c-tauri-lib.rs-badge]][c-tauri-lib.rs]{{hi:tauri}} [![cat-gui][cat-gui-badge]][cat-gui]{{hi:GUI}} [![cat-web-programming][cat-web-programming-badge]][cat-web-programming]{{hi:Web programming}}

Make tiny, secure apps for all desktop platforms with Tauri. [`tauri`][c-tauri]⮳ is an app construction toolkit that lets you build software for all major desktop operating systems using web technologies. Electron-like web-based UI, except it uses system webviews rather than shipping chromium, and non-UI code is written in Rust rather than `node.js`.

```rust,editable
{{#include ../../../deps/tests/categories/gui/tauri.rs:example}}
```

## `dioxus` {#dioxus}

[![dioxus-website][c-dioxus-website-badge]][c-dioxus-website] [![dioxus][c-dioxus-badge]][c-dioxus] [![dioxus-crates.io][c-dioxus-crates.io-badge]][c-dioxus-crates.io] [![dioxus-github][c-dioxus-github-badge]][c-dioxus-github] [![dioxus-lib.rs][c-dioxus-lib.rs-badge]][c-dioxus-lib.rs]{{hi:dioxus}}{{hi:Dom}}{{hi:Gui}}{{hi:React}}{{hi:Ui}}{{hi:Wasm}}

Portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust. A very nice API layer that has Tauri, Web, and TUI renderers. A native renderer is coming soon.

```rust,editable
{{#include ../../../deps/tests/categories/gui/dioxus.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P2 write
</div>
