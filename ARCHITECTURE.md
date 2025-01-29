# Architecture of ZKsync-error


This workspace consists of the following crates:

1. `zksync-error-codegen` -- code generation logic for different backends (Rust, MDbook, TypeScript etc). Can be used as a library or through CLI.
2. `zksync-error-codegen-cli` -- command-line interface to launch code generation.
3. `zksync-error-description` -- public model of error hierarchy used by the *generated Rust code*. It allows the generated code to provide access to the documentation for each error in runtime.
4. `zksync-error-model` -- internal model of error hierarchy used by `zksync-error-codegen` and utility features.

