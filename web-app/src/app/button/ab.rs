use tpp_core::button::{Button, ButtonEvent, ButtonState};
use yew::prelude::*;

use crate::app::button::Btn;

#[derive(Debug)]
pub struct AB {
    props: ABProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ABProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    pub a: ButtonState,
    pub b: ButtonState,
}

impl Component for AB {
    type Message = ButtonEvent;
    type Properties = ABProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        AB { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let render = match (msg.button, msg.state) {
            (Button::A, state) if self.props.a != state => {
                self.props.a = state;
                true
            }
            (Button::B, state) if self.props.b != state => {
                self.props.b = state;
                true
            }
            _ => false,
        };

        if render {
            self.props.onbuttonevent.emit(msg);
        }

        render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let onbuttonevent = self.link.callback(|event| event);

        html! {
            <div class="tpp-ab">
                <Btn onbuttonevent=&onbuttonevent mapping=Button::A state=self.props.a />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::B state=self.props.b />
            </div>
        }
    }
}
