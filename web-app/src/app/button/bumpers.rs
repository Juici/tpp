use tpp_core::button::{Button, ButtonEvent, ButtonState};
use yew::prelude::*;

use crate::app::button::Btn;

#[derive(Debug)]
pub struct Bumpers {
    props: BumpersProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct BumpersProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    pub l: ButtonState,
    pub r: ButtonState,
}

impl Component for Bumpers {
    type Message = ButtonEvent;
    type Properties = BumpersProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Bumpers { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let render = match (msg.button, msg.state) {
            (Button::L, state) if self.props.l != state => {
                self.props.l = state;
                true
            }
            (Button::R, state) if self.props.r != state => {
                self.props.r = state;
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
            <div class="tpp-bumpers">
                <Btn onbuttonevent=&onbuttonevent mapping=Button::L state=self.props.l />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::R state=self.props.r />
            </div>
        }
    }
}
