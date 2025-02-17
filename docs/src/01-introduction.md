# Introduction

Project `zksync-error` aims at describing user-facing failures in ZKsync
components and generating their definitions and documentation for Rust and
Typescript modules.

This description is distributed across multiple interconnected JSON files hosted
in repositories of ZKsync components, allowing for asynchronous work on them. 

## Motivation 

- Provide a smooth and uniform developer and user experience by agreeing on a
  single source of truth across different code bases, the rules of their pretty printing, and their description.
- Retain the ability to work on parts of different ZKsync components separately for smooth adoption.
- Provide ZKsync components with access to the documentation in runtime so that
  they could react to failures and guide users towards solutions.

Every component of ZKsync has, and will have their own ways of handling internal
failures.  `zksync-error` is only concerned with providing an abstraction layer
for outwards facing failures, visible to users.

## Project structure 

The project contains the following crates:

1. `zksync-error-codegen` -- library hosting code generation logic for different
   backends (Rust, MDbook, TypeScript etc).
2. `zksync-error-codegen-cli` -- command-line interface for `zksync-error-codegen`.
3. `zksync-error-description` -- public model of error hierarchy used by the
   *generated Rust code*. It allows the generated code to provide full
   documentation for errors in runtime.
4. `zksync-error-model` -- internal model of error hierarchy used by
   `zksync-error-codegen`.

The code generator of the crate `zksync-error-codegen` generates a crate
`zksync-error` which is included in the workspaces of ZKsync components such as
`anvil-zksync`.

