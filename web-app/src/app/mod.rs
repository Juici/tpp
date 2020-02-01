mod button;

use yew::format::Bincode;
use yew::prelude::*;
use yew::services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tpp_core::{Button, ButtonEvent};

use self::button::Btn;

const WS_ENDPOINT: &str = "ws://localhost:9001/";

pub struct App {
    ws_service: WebSocketService,
    link: ComponentLink<Self>,
    ws: Option<WebSocketTask>,
}

pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<WsResponse>),
}

pub enum WsAction {
    Connect,
    //    Disconnect,
    ConnectionEstablished,
    ConnectionLost,
    ButtonEvent(ButtonEvent),
}

#[derive(Debug, Serialize)]
pub enum WsRequest {
    ButtonEvent(ButtonEvent),
}

#[derive(Debug, Deserialize)]
pub enum WsResponse {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, mut link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::WsAction(WsAction::Connect));

        App {
            ws_service: WebSocketService::new(),
            link,
            ws: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsAction(action) => self.ws_action(action),
            Msg::WsReady(response) => self.ws_ready(response),
        }
    }

    fn view(&self) -> Html {
        let onbuttonevent = self
            .link
            .callback(|event| Msg::WsAction(WsAction::ButtonEvent(event)));

        html! {
            <div class="tpp-app">
                <div class="tpp-dpad">
                    <Btn onbuttonevent=&onbuttonevent mapping=Button::Up />
                    <Btn onbuttonevent=&onbuttonevent mapping=Button::Down />
                    <Btn onbuttonevent=&onbuttonevent mapping=Button::Left />
                    <Btn onbuttonevent=&onbuttonevent mapping=Button::Right />
                </div>
            </div>
        }
    }
}

impl App {
    fn ws_action(&mut self, action: WsAction) -> ShouldRender {
        match action {
            WsAction::Connect => {
                log::debug!("connecting from websocket");

                let callback = self.link.callback(|Bincode(data)| Msg::WsReady(data));

                let notification = self.link.callback(|status| match status {
                    WebSocketStatus::Opened => Msg::WsAction(WsAction::ConnectionEstablished),
                    WebSocketStatus::Closed | WebSocketStatus::Error => {
                        Msg::WsAction(WsAction::ConnectionLost)
                    }
                });

                let task = self
                    .ws_service
                    .connect(WS_ENDPOINT, callback, notification)
                    .unwrap();
                self.ws = Some(task);
            }
            //            WsAction::Disconnect => {
            //                log::debug!("disconnected from websocket");
            //
            //                self.ws.take();
            //            }
            WsAction::ConnectionEstablished => {
                log::debug!("connected from websocket");

                return false;
            }
            WsAction::ConnectionLost => {
                log::debug!("lost connection to websocket");

                self.ws = None;
            }
            WsAction::ButtonEvent(event) => {
                log::debug!("button event: {:?}", event);

                let request = WsRequest::ButtonEvent(event);
                self.ws.as_mut().unwrap().send_binary(Bincode(&request));
            }
        }

        true
    }

    fn ws_ready(&mut self, response: Result<WsResponse>) -> ShouldRender {
        log::debug!("ws_ready: {:?}", response.ok());

        true
    }
}
