# JSON file types

There are several types of descriptions hosted in JSON files:

1. Full files (describe type bindings, domains and some of their components and errors).
2. Domain files (describe a single domain and some of its components and errors).
3. Component files (describe a single component and some of its errors).
4. Error files (describe one or more errors of a single component).

## Full files

Full files contain full hierarchy description with additional information about type mappings:

```json
{
  "types": [],
  "domains": []
}
```
At least one domain and at least one component should be defined.

Such files may be:

- referenced by other files through `takeFrom` fields;
- used as the root error definition file;
- provided as additional JSON files through CLI or library interfaces.

Only full files can be used as the description root.

## Domain files

Domain files contain definition of a single component and some of its errors, for example:

```json
{
  "domain_name": "Core",
  "domain_code": 1,
  "identifier_encoding": "core",
  "description": "Errors in core ZKsync components such as sequencer or mempool.",
  "bindings": {
    "rust": "Core"
  },
  "components": []
}
```

Domain files may be:

- referenced by other files through `takeFrom` fields.
- provided as additional JSON files through CLI or library interfaces.

When all input files and their dependencies are collected and merged, all
domains should have at least one component.

## Component files

Component files: contain definition of a single component and some of its errors, for example. 

```json
{
  "component_name": "API",
  "component_code": 4,
  "identifier_encoding": "api",
  "errors": []
}
```

Such files may be:

- referenced by other files through `takeFrom` fields.
- provided as additional JSON files through CLI or library interfaces. In this case they will be merged with the already defined component of the same name/code/identifier.

## Error list files

Error list files contain lists of errors, for example:

```json

[
  {
    "name": "LoadingStateOverExistingStateError",
    "code": 1,
    "message": "Loading state into a node with existing state is not allowed.",
    "doc": {
      "summary": "It is not allowed to load a state overriding the existing node state.",
      "description": "It is not allowed to load a state overriding the existing node state. If you have a use case for that, please create an issue."
    }
  },
  {
    "name": "StateDecompressionError",
    "code": 3,
    "message": "Failed to decompress state: {details}.",
    "fields": [
      {
        "name": "details",
        "type": "string"
      }
    ]
  }
]
```
