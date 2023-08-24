use tungstenite::connect;
use serde_json::Result;
use gosumemory_helper::Gosumemory;
fn main() -> Result<()> {
    let (mut socket, _response) = connect("ws://localhost:24050/ws").expect("Can't connect");
    if let Ok(msg) = socket.read_message(){
    let json_result: Gosumemory = serde_json::from_str(msg.to_text().unwrap())?;
    println!("{:#?}", json_result);
    }
    Ok(())
}