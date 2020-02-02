mod agent;
mod button;

use yew::prelude::*;

use self::agent::{NetworkAgent, WsRequest, WsResponse};
use self::button::DPad;

pub enum Msg {
    WsResponse(WsResponse),
    WsRequest(WsRequest),
}

pub struct App {
    agent: Box<dyn Bridge<NetworkAgent>>,
    link: ComponentLink<Self>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|response| Msg::WsResponse(response));
        let agent = NetworkAgent::bridge(callback);

        App { agent, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::WsResponse(response) => self.ws_response(response),
            Msg::WsRequest(request) => self.ws_request(request),
        }
    }

    fn view(&self) -> Html {
        let onbuttonevent = self
            .link
            .callback(|event| Msg::WsRequest(WsRequest::ButtonEvent(event)));

        html! {
            <div class="tpp-app">
                <DPad onbuttonevent=&onbuttonevent />
            </div>
        }
    }
}

impl App {
    fn ws_request(&mut self, request: WsRequest) -> ShouldRender {
        let render = match &request {
            WsRequest::ButtonEvent(_) => false,
        };

        self.agent.send(request);
        render
    }

    fn ws_response(&mut self, response: WsResponse) -> ShouldRender {
        match response {
            WsResponse::WsStatusChanged(status) => {
                log::debug!("websocket status changed: {:?}", status)
            }
        }

        false
    }
}
