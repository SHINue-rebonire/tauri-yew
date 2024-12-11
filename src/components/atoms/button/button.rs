use crate::components::atoms::button::style::button_style;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Warning,
    Success,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Medium,
    Small,
}

#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: Callback<()>,
    pub disabled: bool,
    pub size: ButtonSize,
    pub variant: ButtonVariant,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let button_style = button_style();

    let variant = match props.variant {
        ButtonVariant::Primary => "primary",
        ButtonVariant::Secondary => "secondary",
        ButtonVariant::Warning => "warning",
        ButtonVariant::Success => "success",
    };

    let size = match props.size {
        ButtonSize::Medium => "medium",
        ButtonSize::Small => "small",
    };

    let onclick = {
        let on_click = props.on_click.clone();
        Callback::from(move |_: MouseEvent| {
            on_click.emit(());
        })
    };

    html! {
        <button
            class={classes!(button_style, variant, size)}
            onclick={onclick}
            disabled={props.disabled}
        >
            { &props.label }
        </button>
    }
}
