# Migrating Rust code to ZKsync error

## If the function returns `Result<_,anyhow::Error>`

Suppose you want to throw an error from a component `AnvilEnvironment` of domain `Anvil` instead of `anyhow::Error`. Follow these steps:

1. Change the return type of the function to `Result<_,
   zksync_error::anvil::gen::GenericError>` and it will be automatically cast to
   a `GenericError` of this component.

   On error throwing sites, if the implicit call to `into` is not sufficient or is not applicable, then try to do the following:
   
   - map the error using the function `zksync_error::anvil::gen::to_generic`,
   for example:
   

    ```rust
    //replace the first line with the second
        let log_file = File::create(&config.log_file_path)?;
        // Instead of returning `std::io::Error` you now return `anvil::gen::GenericError` containing it
        let log_file = File::create(&config.log_file_path).map_err(to_generic)?;

    //This is equivalent to `return anyhow!(error)`
    anyhow::bail!(
            "fork is using unsupported fee parameters: {:?}",
            fork_client.details.fee_params
        )

    // We still return `anyhow::Error` but it is cast to `GenericError` of our component
    return Err(anyhow!(
        "fork is using unsupported fee parameters: {:?}",
        fork_client.details.fee_params
    )
    .into())

    // Instead of `into` we may use a macro `generic_error` imported from `anvil::gen::generic_error`, or from a namespace of a different component.
    return Err(generic_error!(
        "fork is using unsupported fee parameters: {:?}",
        fork_client.details.fee_params
    ))
    ```
  
2. Introduce more errors, corresponding to different failure situations.

   Describe the new error in the JSON file, and replace it on the throw site.


   
## If the function returns `Result` with an internal error 

Many functions return `Result<_, CustomErrorType>` and if the error
`CustomErrorType` ends up getting outside the component, you may want to modify
the function to return 
`Result<_, zksync_error::anvil::env::AnvilEnvironmentError>` instead,  or an
error related to a different component. 

Suppose functions `caller1`, `caller2` and so on call a function 
`fn f()->Result<_, CustomErrorType>`, and we want to migrate `f` to 
`AnvilEnvironmentError` step by step.

1. Assess if you *really* need to rewrite it. Remember that all error types
   described in JSON and provided by `zksync_error` are for public errors only!
   If this error gets through the call stack to the component user, then it
   makes sense to migrate it to `ZksyncError`.
2. On the call site, in `caller`, map the error value using
   `map_err(to_generic)` and refactor the `caller` until it compiles.
3. Make a copy of `f`, say, `f_new`.
4. Replace the return type in `f_new` with `Result<_, AnvilEnvironmentError>`. 
   Whenever `f_new` returns `Err(e:CustomErrorType)`,  it may return `anvil::env::GenericError` instead.
   Use `.map_err(to_generic)`, explicit or implicit `into`, and `anvil::env::generic_error!` macros.
5. Work through the callers, one at a time. Pick a caller, say, `caller1`.
   Replace calls to `f` with calls to `f_new` and remove unused conversions. 
6. Repeat until all callers are refactored.
7. Remove the function `f` and rename `f_new` back to `f`.
   


## Functions returning errors from multiple components

If a function may return errors that are spread among components (e.g. either
`AnvilEnvironment` or `AnvilStateLoader` components) then we advise to return a
type `AnvilError` from it. 
Such function are rare and require more manual work, but the algorithm is the
same -- just use `to_domain` helper along with `generic_error!` macro.

- Choosing between CLI and `build.rs`

## Publishing

After you change your repository's JSON file, rebuild `anvil-zksync` and `zksync-error` crate
will be regenerated.
You may also build `zksync-error` crate using CLI.


When you publish your `anvil.json` to the repository, other projects will see
the updated definitions. Then they should regenerate their `zksync-error` crates
to use them. 
