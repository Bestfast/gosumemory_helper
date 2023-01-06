# gosumemory_helper

small library to parse [gosumemory](https://github.com/l3lackShark/gosumemory) websocket messages, made in Rust as a learning project.

### Usage

Use [tungstenite](https://docs.rs/crate/tungstenite/0.18.0) to connect to the webserver, then parse the message with the json library.

Use the JsonValue returned when parsing the message to construct the various helpers.

```Rust
use gosumemory_helper::Metadata;
let metadata = Metadata::new(&json_result).unwrap();
println!("Map title: {}", metadata.title);
```

Docs will be available soon:tm: but since it's a small library, you can look at the code yourself to see the available methods and structs. 