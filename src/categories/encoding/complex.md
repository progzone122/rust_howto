# Structured Data

{{#include complex.incl.md}}

## Serialize and deserialize unstructured JSON {#serde-json}

[![serde_json][c-serde_json-badge]][c-serde_json]{{hi:serde_json}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:Deserialization}}{{hi:JSON}}

The [`serde_json`][c-serde_json]{{hi:serde_json}}⮳ crate provides a [`serde_json::from_str`][c-serde_json::from_str]{{hi:serde_json::from_str}}⮳ function to parse{{hi:Parsing}} a `&str` of JSON.

Unstructured JSON can be parsed into a universal [`serde_json::Value`][c-serde_json::Value]{{hi:serde_json::Value}}⮳ type that is able to represent any valid JSON data.

The example below shows a `&str` of JSON being parsed. The expected value is declared using the [`serde_json::json`][c-serde_json::json]{{hi:serde_json::json}}⮳ macro.

```rust,editable
{{#include ../../../deps/tests/categories/encoding/json.rs:example}}
```

## Deserialize a TOML configuration file {#toml}

[![toml][c-toml-badge]][c-toml]{{hi:toml}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:TOML}}

Parse some TOML into a universal `toml::Value` that is able to represent any valid TOML data.

```rust,editable
{{#include ../../../deps/tests/categories/encoding/toml.rs:example}}
```

Parse TOML into your own structs using [`serde`][c-serde]{{hi:serde}}⮳.

```rust,editable
{{#include ../../../deps/tests/categories/encoding/toml1.rs:example}}
```

## Read and write integers in little-endian byte order {#byteorder}

[![byteorder][c-byteorder-badge]][c-byteorder]{{hi:byteorder}} [![cat-encoding][cat-encoding-badge]][cat-encoding]{{hi:Encoding}}{{hi:Little endian}}{{hi:Byte order}}

[`byteorder`][c-byteorder]⮳ can reverse the significant bytes of structured data. This may be necessary when receiving information over the network, when bytes received are from another system.

```rust,editable
{{#include ../../../deps/tests/categories/encoding/endian_byte.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO P1 clean up toml
</div>
