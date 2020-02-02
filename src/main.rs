mod input;

use std::thread;

use anyhow::Result;
use std::sync::mpsc::Sender;
use tpp_core::button::ButtonEvent;
use tpp_core::ws::WsRequest;
use websocket::sync::{Client, Server};
use websocket::{Message, OwnedMessage};

const WS_ADDRESS: &str = "localhost:9001";

fn main() -> Result<()> {
    pretty_env_logger::init();

    let input = input::spawn_handler();

    let server = Server::bind(WS_ADDRESS)?;

    for connection in server.filter_map(Result::ok) {
        let input = input.clone();

        thread::spawn(move || {
            let input = input;

            let client: Client<_> = connection.accept().unwrap();

            let (mut rx, mut tx) = client.split().unwrap();

            for message in rx.incoming_messages() {
                if let Err(err) = handle_message(&input, message) {
                    log::error!("{:?}", err);

                    let _ = tx.send_message(&Message::close());
                    return;
                }
            }
        });
    }

    Ok(())
}

fn handle_message(
    input: &Sender<ButtonEvent>,
    message: websocket::WebSocketResult<OwnedMessage>,
) -> Result<()> {
    match message? {
        OwnedMessage::Binary(msg) => {
            let msg: WsRequest = bincode::deserialize(&msg)?;

            log::trace!("handle request: {:?}", msg);

            match msg {
                WsRequest::ButtonEvent(event) => {
                    log::trace!("button event: {:?}", event);
                    input.send(event)?;
                }
            }
        }
        _ => {}
    }

    Ok(())
}
