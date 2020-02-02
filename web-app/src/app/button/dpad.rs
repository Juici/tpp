use tpp_core::button::{Button, ButtonEvent, ButtonState};
use yew::prelude::*;

use crate::app::button::Btn;

#[derive(Debug)]
pub struct DPad {
    props: DPadProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DPadProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    pub up: ButtonState,
    pub down: ButtonState,
    pub left: ButtonState,
    pub right: ButtonState,
}

impl Component for DPad {
    type Message = ButtonEvent;
    type Properties = DPadProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DPad { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let render = match (msg.button, msg.state) {
            (Button::Up, state) if self.props.up != state => {
                self.props.up = state;
                true
            }
            (Button::Down, state) if self.props.down != state => {
                self.props.down = state;
                true
            }
            (Button::Left, state) if self.props.left != state => {
                self.props.left = state;
                true
            }
            (Button::Right, state) if self.props.right != state => {
                self.props.right = state;
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
            <div class="tpp-dpad">
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Up state=self.props.up />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Down state=self.props.down />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Left state=self.props.left />
                <Btn onbuttonevent=&onbuttonevent mapping=Button::Right state=self.props.right />
            </div>
        }
    }
}
