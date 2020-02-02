use tpp_core::button::{Button, ButtonEvent, ButtonState};
use yew::prelude::*;

use crate::app::button::Btn;

#[derive(Debug)]
pub struct Options {
    props: OptionsProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct OptionsProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    pub start: ButtonState,
    pub select: ButtonState,
}

impl Component for Options {
    type Message = ButtonEvent;
    type Properties = OptionsProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Options { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let render = match (msg.button, msg.state) {
            (Button::Start, state) if self.props.start != state => {
                self.props.start = state;
                true
            }
            (Button::Select, state) if self.props.select != state => {
                self.props.select = state;
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
            <div class="tpp-options">
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Start state=self.props.start />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Select state=self.props.select />
            </div>
        }
    }
}
