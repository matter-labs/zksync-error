# Types of errors


The root type of the error hierarchy is `ZksyncError`. For example, the error
`GenericError` from the domain `Anvil` and its component `AnvilEnvironment`
is instantiated as follows:

```rust
    ZksyncError::Anvil(// Domain
        Anvil::AnvilEnvironment ( // Component
            AnvilEnvironment::GenericError { // Error
                message: "hello!".to_string()
            })
    );
```

You don't need to construct this instance explicitly -- there are adapters
implementing `Into` for every use case:

1. from errors to domain errors types e.g. `Anvil`;
2. from errors to component errors types e.g. `AnvilEnvironment`;
3. from errors to the root type `ZksyncError`;
4. from domain errors to `ZksyncError`;
5. from `anyhow::Error` to component errors.

See the examples below:

```rust
// This works:
let err : ZksyncError = AnvilEnvironment::GenericError {
                message: "hello!".to_string()
            }.into();


// instead of passing type to `into` where type derivation does not work
let err_same = AnvilEnvironment::GenericError {
                message: "hello!".to_string()
            }.to_unified();
            
// Also works:
fn test() -> Result<(), ZksyncError> {
    Err(AnvilEnvironmentError::GenericError {
        message: "Oops".to_string(),
    })?;
    Ok(())
}

// Also works:
fn test_domain() -> Result<(), AnvilError> {
    Err(AnvilEnvironmentError::GenericError {
        message: "Oops".to_string(),
    })?;
    Ok(())
}

// anyhow -> component, produces the default GenericError belonging to this component
fn test_anyhow() -> Result<(), AnvilEnvironmentError> {
    Err(anyhow::anyhow!("Oops"))?;
    Ok(())
}

// 
fn test_domain() -> Result<(), AnvilError> {
    Err(AnvilEnvironmentError::GenericError {
        message: "Oops".to_string(),
    })?;
    Ok(())
}
```

## Accessing errors

Use the path `zksync_error::<domain_identifier>::<component_identifier>::<error_name>.
 
For example, suppose we have defined a domain `Anvil` with two components
`Environment` and `Generic`. Their identifiers The following example outlines
the structure of the interface, in Rust pseudocode:


```rust
pub mod anvil {
    type AnvilError;
    pub mod env {
        type AnvilEnvironmentError;
        type GenericError;
        type FirstCustomError;
        macro generic_error;
        // converts anything that implements `to_string` into a generic error inside this component.
        fn to_generic<E: Display>(error); 
    }
    pub mod gen {
    ...
    }
}
```
