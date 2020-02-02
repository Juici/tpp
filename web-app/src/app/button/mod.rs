pub mod ab;
pub mod bumpers;
pub mod dpad;
pub mod options;

//use stdweb::web::event::MouseButton;
use tpp_core::button::{Button, ButtonEvent, ButtonState};
use yew::prelude::*;

#[derive(Debug)]
pub struct Btn {
    props: BtnProps,
    link: ComponentLink<Self>,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BtnProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    #[props(required)]
    pub mapping: Button,
    #[props(required)]
    pub state: ButtonState,
}

impl Component for Btn {
    type Message = ButtonState;
    type Properties = BtnProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Btn { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if self.props.state != msg {
            self.props.state = msg;
            self.props.onbuttonevent.emit(ButtonEvent {
                state: self.props.state,
                button: self.props.mapping,
            });
        }

        true
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
        let base_classes = ["tpp-btn", self.props.mapping.class_name()];

        let mut classes = Vec::with_capacity(3);
        classes.extend_from_slice(&base_classes);
        if self.props.state == ButtonState::Down {
            classes.push("tpp-btn-down");
        }

        html! {
            <div
                class=classes
//                ontouchstart=self.link.callback(|_| ButtonState::Down)
//                ontouchenter=self.link.callback(|_| ButtonState::Down)
//                ontouchend=self.link.callback(|_| ButtonState::Up)
//                ontouchcancel=self.link.callback(|_| ButtonState::Up)
                onmousedown=self.link.callback(|_| ButtonState::Down)
                onmouseup=self.link.callback(|_| ButtonState::Up)
                onmouseout=self.link.callback(|_| ButtonState::Up)
                >
                <span class="tpp-btn-icon">{ self.props.mapping.text() }</span>
            </div>
        }
    }
}

pub trait ButtonExt: Copy {
    fn class_name(self) -> &'static str;
    fn text(self) -> &'static str;
}

impl ButtonExt for Button {
    fn class_name(self) -> &'static str {
        match self {
            Button::Select => "tpp-btn-select",
            Button::Start => "tpp-btn-start",
            Button::Up => "tpp-dpad-up",
            Button::Down => "tpp-dpad-down",
            Button::Left => "tpp-dpad-left",
            Button::Right => "tpp-dpad-right",
            Button::A => "tpp-btn-a",
            Button::B => "tpp-btn-b",
            Button::L => "tpp-bumper-l",
            Button::R => "tpp-bumper-r",
        }
    }

    fn text(self) -> &'static str {
        match self {
            Button::Select => "SELECT",
            Button::Start => "START",
            Button::Up => "",
            Button::Down => "",
            Button::Left => "",
            Button::Right => "",
            Button::A => "A",
            Button::B => "B",
            Button::L => "L",
            Button::R => "R",
        }
    }
}
