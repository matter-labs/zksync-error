# ZKsync-error

A unified description of possible failures in the components of ZKsync, along with error formats and diagnostic messages.

[![Logo](eraLogo.png)](https://zksync.io/)

- A description of all failures that can be observed in ZKsync components.
- Organized in a 3-level hierarchy: domains, their components, and failures in these components.
- For each possible failure, the description includes the error message, its fields, and full documentation.
- The description may be split into multiple files stored in multiple repositories, each project may independently develop their own component.
- The crate `zksync-error-codegen` is able to generate Rust code to handle these errors, along with documentation in MDBook format. In the future, a TypeScript backend will also be supported.

[Architecture of ZKsync-error](ARCHITECTURE.md)

## Policies

- [Security policy](SECURITY.md)
- [Contribution policy](CONTRIBUTING.md)

## License

ZKsync Era is distributed under the terms of either

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/blog/license/mit/>)

at your option.
