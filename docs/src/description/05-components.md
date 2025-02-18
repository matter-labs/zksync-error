# Components

Every component implicitly contains an error with code 0 meaning an umbrella
"generic error". You usually should not define this error yourself.

```json
{
  "name": "GenericError",
  "code": 0,
  "message": "Generic error: {message}",
  "fields": [
    {
      "name": "message",
      "type": "string"
    }
  ]
}
```
