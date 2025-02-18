# Types

Errors are data structures, they contain named fields. Fields may have one of
the types  defined in the JSON files in the section `types` next to `domains`,
for example:

```json
"types": [
    {
        "name": "uint",
        "description": "Unsigned 32-bit integer",
        "bindings": {
            "rust":
            {
                "name": "u32",
                "path": ""
            }
        }
    },
    {
        "name": "int",
        "description": "Signed 32-bit integer",
        "bindings": {
            "rust":
            {
                "name": "i32",
                "path": ""
            }
        }
    },

```

Initially, thanks to the definitions provided in `zksync-error`, error fields
can have one of the following types:

- `string`;
- `int` for 32-bit signed integers;
- `uint` for 32-bit unsigned integers;
- `wrapped_error`: a JSON value serialized through `serde_json`.
- `map`: a map from `string` to `string`;
- `bytes`: a sequence of bytes:
- type of any other error `E`, defined in one of JSONs. In Rust it is mapped to
  `Box<E>`. This allows wrapping one error inside another.

Errors in `zksync-error` are part of a component's interface, so they can not
have types that are unknown to other components.
This prevents directly wrapping errors that are defined internally in one of components.
