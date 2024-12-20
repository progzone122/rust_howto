# Attributes

{{#include attributes.incl.md}}

## Attributes {#attributes}

{{i:Attributes}} can take arguments with different syntaxes:

```rust,editable,compile_fail,noplayground
#[attribute = "value"]
#[attribute(key = "value")]
#[attribute(value)]
#[attribute(value, value2)]
```

Inner attributes{{hi:Attributes}} `#![attr]` apply to the item that the attribute is declared within.

## Lint attributes {#lint-attributes}

During early development, place the following attributes at the top of `main.rs` or `lib.rs`

```rust,editable
{{#include ../../deps/tests/language/attributes_debug.rs:example}}
```

For production-ready code{{hi:Production-ready code}}, replace the above by the following, for example.

```rust,editable
{{#include ../../deps/tests/language/attributes_production.rs:example}}
```

You also apply these attributes to specific functions:

```rust,editable
{{#include ../../deps/tests/language/allow_dead_code.rs:example}}
```

List of lint checks{{hi:Lint checks}}: `rustc -W help`. [`rustc`][rustc]{{hi:rustc}}⮳ also recognizes the tool lints for "clippy" and "rustdoc" e.g. `#![warn(clippy::pedantic)]`

## Automatically derive common traits {#automatic-trait-derivation}

See [Automatic derivation][p-automatic-derivation].

## Mark as `must use` {#must-use}

```rust,editable
{{#include ../../deps/tests/language/attributes_must_use.rs:example}}
```

## Mark as deprecated {#deprecated}

```rust,editable
{{#include ../../deps/tests/language/attributes_deprecated.rs:example}}
```

## Compile conditionally {#conditional-compilation}

[Conditional compilation][book-rust-reference-conditional-compilation]⮳

```rust,editable
{{#include ../../deps/tests/language/conditional_compilation.rs:example}}
```

## See also

[Attributes reference][book-rust-reference-attributes]⮳

[![Rust by example - attributes][book-rust-by-example-attributes-badge]][book-rust-by-example-attributes]

[p-automatic-derivation]: ../standard-library/derive.md
{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
</div>
