use stdweb::web::event::MouseButton;
use tpp_core::{Button, ButtonEvent, ButtonState};
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

        let onmouseover = self.link.callback(|event: MouseOverEvent| {
            if event.buttons().is_down(MouseButton::Left) {
                ButtonState::Down
            } else {
                ButtonState::Up
            }
        });

        let onpointerover = self.link.callback(|event: PointerOverEvent| {
            if event.pressure() > 0.0 {
                ButtonState::Down
            } else {
                ButtonState::Up
            }
        });

        html! {
            <div
                class=classes
                ontouchstart=self.link.callback(|_| ButtonState::Down)
                ontouchenter=self.link.callback(|_| ButtonState::Down)
                ontouchend=self.link.callback(|_| ButtonState::Up)
                ontouchcancel=self.link.callback(|_| ButtonState::Up)
                onmousedown=self.link.callback(|_| ButtonState::Down)
                onmouseup=self.link.callback(|_| ButtonState::Up)
                onmouseout=self.link.callback(|_| ButtonState::Up)
                onmouseover=&onmouseover
                onpointerover=&onpointerover
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
