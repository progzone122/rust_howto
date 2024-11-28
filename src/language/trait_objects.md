# Trait Objects {#trait-objects}

{{#include trait_objects.incl.md}}

In Rust, traits{{hi:Traits}} are types, but they are "unsized"{{hi:Unsized}}, which roughly means that they are only allowed to show up behind a pointer like [`std::boxed::Box`][c-std::boxed::Box]{{hi:std::boxed::Box}}⮳ (which points onto the heap) or `&` (which can point anywhere).

A type like `&ClickCallback` or `Box<dyn ClickCallback>` where `ClickCallback` is a Trait is called a "trait object", and includes a pointer to an instance of a type `T` implementing `ClickCallback`, and a vtable{{hi:Vtable}}: a pointer to `T`'s implementation of each method in the trait.

```rust,editable
{{#include ../../deps/tests/lang/trait_objects.rs:example}}
```

The set of traits after [`dyn`][keyword-dyn]{{hi:dyn}} is made up of an [object-safe-reference][book-rust-reference-object-safe]⮳ base trait plus any number of autotraits{{hi:Autotraits}} (one of [`std::marker::Send`][c-std::marker::Send]{{hi:std::marker::Send}}⮳, [`std::marker::Sync`][c-std::marker::Sync]{{hi:std::marker::Sync}}⮳, [`std::marker::Unpin`][c-std::marker::Unpin]{{hi:std::marker::Unpin}}⮳, [`std::panic::UnwindSafe`][c-std::panic::UnwindSafe]{{hi:std::panic::UnwindSafe}}⮳, and [`std::panic::RefUnwindSafe`][c-std::panic::RefUnwindSafe]{{hi:std::panic::RefUnwindSafe}}⮳ - see [special traits][book-rust-reference-special-traits]⮳).

```rust,editable,compile_fail
dyn Trait
dyn Trait + Send
dyn Trait + Send + Sync
dyn Trait + 'static
```

## See also

[Trait Objects (docs)][book-rust-trait-objects]{{hi:Trait objects}}⮳

{{#include refs.incl.md}}
{{#include ../refs/link-refs.md}}

<div class="hidden">
TODO P1: review
</div>