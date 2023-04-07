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
