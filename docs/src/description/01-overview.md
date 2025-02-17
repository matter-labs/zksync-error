# Overview

Error and their documentation are described in JSON files. 

- Every error has a *code*, belongs to a *component*, and every component
  belongs to a *domain*. This structure matches the structure of JSON files. For example, this file describes one domain `Anvil` with one component `AnvilEnvironment` and one
  error `LogFileAccessError`:

```json
{
    "types": [],
    "domains": [
        {
            "domain_name": "AnvilZksync",
            "domain_code": 5,
            "identifier_encoding": "anvil_zksync",
            "components" : [
                {
                    "component_name": "AnvilEnvironment",
                    "component_code": 1,
                    "identifier_encoding": "env",
                    "errors" : [
                        {
                            "name": "LogFileAccessError",
                            "code": 1,
                            "message": "Unable to access log file: {log_filename}",
                            "fields": [
                                {
                                    "name": "log_filename",
                                    "type": "string"
                                },
                                {
                                    "name": "wrapped_error",
                                    "type": "string"
                                }
                            ]
                       }
                    ]
                }
            ]
        }
        ]
}
```

- One file is considered a *root* file, and is used as a start to collect
  information about all errors.

- The root file should contain high levels of hierarchy: domains, components. It
  may define errors, but it may also link definitions of domains and components
  in other files through the mechanism of [links](07-links.md).

- The root file is merged with all files it links to (and their linked files, transitively), and with any additional files provided by the user through CLI or library interfaces.

- Each linked file may contain a fragment of a full error hierarchy with
  domains, components, and errors, or a smaller fragment e.g. just a list of errors.


Let us now walk through the levels of this hierarchy, from errors towards domains.
