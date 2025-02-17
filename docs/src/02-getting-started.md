# Getting started

The main JSON file is located in [zksync-error
repository](https://github.com/matter-labs/zksync-error/blob/main/zksync-root.json).
It is also baked inside the `zksync-error-codegen` crate, so that when you pin a
version of `zksync-error` you also pin a version of `zksync-root.json` file,
accessible to the codegen through the link `zksync-errors://zksync-root.json`.

It is linked to similar files in other repositories through `takeFrom` fields.
These files are automatically merged to produce a single tree of
domains-components-errors.  This tree-like model can be used to generate:
- `zksync-error` crate, defining errors for Rust code;
- documentation for errors in MDbook format.
- in future, TypeScript code to interact with these errors.

This approach allows to define errors in the projects where they are used, for
example in Solidity compiler or VM.
