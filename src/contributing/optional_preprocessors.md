## Optional pre-processors {#optional-preprocessors}

{{#include optional_preprocessors.incl.md}}

- [`mdbook-linkcheck`][c-mdbook_linkcheck-github]{{hi:mdbook-linkcheck}}⮳ is a backend for [`mdbook`][c-mdbook]{{hi:mdbook}}⮳ that will check links. Install with `cargo install mdbook-linkcheck`. Uncomment the related section in `book.toml`.

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: add

- [`mdbook-hide`][c-mdbook_hide-github]{{hi:mdbook-hide}}⮳ hides chapters under construction. Install with `cargo install mdbook-hide`. Uncomment the related section in `book.toml`. To mark a chapter as hidden, add the following comment anywhere in the Markdown file. It is better to have it at the top of the file for clarity.

```xml
<!--hidden-->
```

- [`mdbook-keeper`][c-mdbook_keeper-crates.io]{{hi:mdbook-keeper}}⮳. Install with

```bash
cargo install mdbook_keeper --git https://github.com/tfpk/mdbook_keeper.git
```

</div>
