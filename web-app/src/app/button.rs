use yew::prelude::*;

use tpp_core::{Button, ButtonEvent, ButtonState};

pub struct Btn {
    link: ComponentLink<Self>,
    onbuttonevent: Callback<ButtonEvent>,
    mapping: Button,
    state: ButtonState,
}

#[derive(Clone, Properties)]
pub struct BtnProps {
    #[props(required)]
    pub onbuttonevent: Callback<ButtonEvent>,
    #[props(required)]
    pub mapping: Button,
    pub state: ButtonState,
}

impl Component for Btn {
    type Message = ButtonState;
    type Properties = BtnProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Btn {
            link,
            onbuttonevent: props.onbuttonevent,
            mapping: props.mapping,
            state: props.state,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        let render = self.state != msg;

        self.state = msg;
        self.onbuttonevent.emit(ButtonEvent {
            state: self.state,
            button: self.mapping,
        });

        render
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        self.mapping = props.mapping;
        self.state = props.state;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div
                class=("tpp-btn", self.mapping.class_name())
                ontouchstart=self.link.callback(|_| ButtonState::Down)
                ontouchend=self.link.callback(|_| ButtonState::Up)
                onmousedown=self.link.callback(|_| ButtonState::Down)
                onmouseup=self.link.callback(|_| ButtonState::Up)>
                <span class="tpp-btn-icon">{ self.mapping.text() }</span>
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
            Button::L => "tpp-btn-l",
            Button::R => "tpp-btn-r",
        }
    }

    fn text(self) -> &'static str {
        match self {
            Button::Select => "Select",
            Button::Start => "Start",
            Button::Up => "⯅",
            Button::Down => "⯆",
            Button::Left => "⯇",
            Button::Right => "⯈",
            Button::A => "A",
            Button::B => "B",
            Button::L => "L",
            Button::R => "R",
        }
    }
}
