# DiffLogger

generates a log of changes for JSON values

# Example

```rs
use diff_logger::DiffLogger;
use serde_json::json;

fn main() {
    let logger = DiffLogger::new().set_header("timestamp");

    let prev = json!({
        "name": "David",
        "age": 43,
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
        "email": [
            { "num": "John@email.com"},
        ]
    });

    logger.log_diff(&prev, &next);
}
```

logs:  

```
 ~ name: David -> John
 ~ age: 43 -> 35 | -8
 ~ state(13:17:50 -> 14:17:50 | 1:0 hours)
   + newField: 4
   - removedField: "some text"
   ~ valueChange: 45 -> 42352 | 42307
 ~ email
   ~ 0
     ~ num: david1@email.com -> John@email.com
```