use anyhow::Result;
use tpp_core::ws::{ConnectionStatus, WsRequest, WsResponse};
use yew::format::Bincode;
use yew::prelude::worker::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

const WS_ENDPOINT: &str = "ws://localhost:9001/";

#[derive(Debug)]
pub enum Msg {
    WsResponse(Result<WsResponse>),
    WsStatusChanged(WebSocketStatus),
}

pub struct NetworkAgent {
    connection: Option<WebSocketTask>,
    status: ConnectionStatus,
    handler_id: Option<HandlerId>,
    link: AgentLink<NetworkAgent>,
    service: WebSocketService,
}

impl NetworkAgent {
    fn ws_response(&mut self, response: Result<WsResponse>) {
        log::debug!("ws response: {:?}", response.ok());
    }

    fn update_status(&mut self, status: ConnectionStatus) {
        if status == ConnectionStatus::Disconnected {
            self.connection = None;
        }
        self.status = status;

        if let Some(handler_id) = self.handler_id {
            self.link
                .respond(handler_id, WsResponse::WsStatusChanged(status));
        }
    }
}

impl Agent for NetworkAgent {
    type Reach = Context;
    type Message = Msg;
    type Input = WsRequest;
    type Output = WsResponse;

    fn create(link: AgentLink<Self>) -> Self {
        let mut agent = NetworkAgent {
            connection: None,
            status: ConnectionStatus::Disconnected,
            handler_id: None,
            link,
            service: WebSocketService::new(),
        };

        log::debug!("connecting to websocket");

        let callback = agent.link.callback(|Bincode(data)| Msg::WsResponse(data));
        let notification = agent.link.callback(|status| Msg::WsStatusChanged(status));

        let task = agent
            .service
            .connect(WS_ENDPOINT, callback, notification)
            .unwrap();
        agent.connection = Some(task);

        agent
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            Msg::WsResponse(response) => self.ws_response(response),
            Msg::WsStatusChanged(status) => {
                let status = match status {
                    WebSocketStatus::Closed | WebSocketStatus::Error => {
                        ConnectionStatus::Disconnected
                    }
                    WebSocketStatus::Opened => ConnectionStatus::Connected,
                };
                self.update_status(status);
            }
        }
    }

    fn connected(&mut self, _id: HandlerId) {
        log::debug!("agent bridge connected");
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        match self.connection.as_mut() {
            Some(connection) => connection.send_binary(Bincode(&msg)),
            None => log::error!(
                "not connected to websocket, failed to send message: {:?}",
                msg
            ),
        }
    }

    fn disconnected(&mut self, _id: HandlerId) {
        log::debug!("agent bridge disconnected");
    }
}
