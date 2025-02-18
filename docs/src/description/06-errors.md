# Errors

Let us start with a minimal example of an error with a single field `msg` of type `string`:

```json
 {
    "name": "FailedToAppendTransactionToL2Block",
    "code": 17,
    "message": "Failed to append the transaction to the current L2 block: {msg}",
    "fields": [
        {
            "name": "msg",
            "type": "string"
        }
    ]
}
...
```

## Name

All errors have human-readable names, which may contain alphanumeric characters,
underscores, but should not start with a digit.

## Code 

All errors are assigned a numeric code from 0 to 9999. It is assumed that every
component has an error with a code 0 meaning "a generic error" and named
"GenericError". Therefore the code 0 is reserved for such error; you may still
redefine it, but you have to keep its semantic.

Error codes should be unique inside a single component. Once allocated, the
error code is never reused, even if the error is deprecated.

## Fields

Errors are data structures and may have zero or more fields. See
[Types](./03-types.md) for a reference on the supported field types.

Each type definition contains a human-readable documentation in the field `description` and bindings to `rust` and other languages.


## Identifiers
Errors have identifiers in form
`[<domain_identifier>-<component_identifier>-<error_code>]`. For example, the
error `LogFileAccessError` in the example above has an identifier
`[anvil_zksync-env-1]`:
Identifiers are globally unique and never reused if the error is deprecated.
  

## Message

The description of an error must provide an error message shown to the users.
The message may include error fields referenced by their names: `{field_name}`.

Users will see this message prepended with the error identifier. For example, suppose that the domain `AnvilZksync` (identifier `anvil_zksync`)  contains a component `Node` (identifier `node`) with an error `FailedToAppendTransactionToL2Block`:


```json
{
  "name": "FailedToAppendTransactionToL2Block",
  "code": 17,
  "message": "Failed to append the transaction to the current L2 block: {msg}",
  "fields": [
    {
      "name": "msg",
      "type": "string"
    }
  ]
}
```

Suppose the Rust code emits an error `FailedToAppendTransactionToL2Block` and provides "Specific error message" as a value for field `msg`. Then this error is signaled with a message `[anvil_zksync-node-17] Failed to append the transaction to the current L2 block: Specific error message`.
