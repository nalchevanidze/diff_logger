# DiffLogger

generates a log of changes for JSON values

# Example

```rs
use diff_logger::DiffLogger;
use serde_json::json;

fn main() {
    let logger = DiffLogger::new().set_header("timestamp", false);

    let prev = json!({
        "name": "David",
        "age": 43,
        "ver": {
            "timestamp": "2023-04-07T11:17:50+00:00",
            "value": "some text",
        },
        "state": {
            "timestamp": "2023-04-07T11:17:50+00:00",
            "removedField": "some text",
            "valueChange": 45
        },
        "email": [
            { "num": "david1@email.com"},
        ]
    });

    let next = json!(
    {
        "name": "John",
        "age": 35,
        "state": {
            "timestamp": "2023-04-07T12:17:50+00:00",
            "newField": 4,
            "valueChange": 42352
        },
        "ver": {
            "timestamp": "2023-04-07T11:18:50+00:00",
            "value": "some text",
        },
        "email": [
            { "num": "John@email.com"},
        ]
    });

    logger.log_diff(&prev, &next);
}
```

logs:  

```
~ name: "David" -> "John"
~ email:
  ~ 0:
    ~ num: "david1@email.com" -> "John@email.com"
~ state:◖13:17:50 -> 14:17:50 | 1:0 hours◗
  - removedField: "some text"
  ~ valueChange: 45 -> 42352 | 42307
  + newField: 4
~ ver:◖13:17:50 -> 13:18:50 | 1:0 minutes◗
~ age: 43 -> 35 | -8
```