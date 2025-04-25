# Links between descriptions

Links identify files that host parts of the error hierarchy.

## Types of links

There are three types of links:

- Default links.
  + Format: `zksync-error://<file-name>`. 
  + Resolved to files included in `zksync-error-codegen` crate. 
  + The default link `zksync-error://zksync-root.json` is resolved to the root of error hierarchy backed in currently used version of `zksync-error-codegen`.
- URL links
  + Format: `https://<URL>` or `http://<URL>`
  + Use to refer to the parts of error hierarchy hosted in other repositories
- File system links
  + Format: `file://<local path>` or simply `<path>`
- Cargo links (experimental): 
  + Format: `cargo://<package-name>@`. 
  + Available only when `zksync-error-codegen` is used as a library from `build.rs` file.
  + Require packages to provide manifests pointing at the corresponding files.

## Usage

1. Links identify the starting JSON files:
    - when using CLI -- through option `--source` 
    - when using `zksync-error-codegen` as a library -- through the field
      `input_links` of the structure
      `zksync_error_codegen::arguments::GenerationArguments`:

    ```rust
pub struct GenerationArguments {
    pub verbose: bool,
    pub input_links: Vec<String>,
    pub override_links: Vec<(String, String)>,
    pub outputs: Vec<BackendOutput>,
    }
    ```

2. Domains and components may refer to other files through `take_from` fields: 

  ```json
  {
        "domain_name": "AnvilZKsync",
        "domain_code": 5,
        "identifier_encoding": "anvil_zksync",
        "description": "Errors originating in Anvil for ZKsync.",
        "take_from": [ "https://<url1>", "https://<url2>" ]
    }
  
  ```

  In this case files are fetched, their contents are parsed, filtered and merged
  into the root model. The filtering selects only the domain/component with the
  same values of fields `name`, `code`, and `identifier_encoding`.
  Instead of URLs you may, of course, use any type of links.
