use stylist::{style, Style};

pub fn button_style() -> Style {
    style!(
        r#"
        border: none;
        outline: none;
        display: inline-flex;
        justify-content: center;
        align-items: center;

        &.primary {
            background-color: #0000CC;
            color: #FFF;
        }
        &.primary:hover {
            background-color: #0000FF;
        }
        &.primary:active {
            background-color: #000099;
        }

        &.secondary {
            background-color: #FFF;
            border: 1px solid #0000CC;
            color: #0000CC;
        }
        &.secondary:hover {
            border-color: #0000FF;
            color: #0000FF;
        }
        &.secondary:active {
            background-color: #DDD;
            border-color: #000099;
            color: #000099;
        }

        &.warning {
            background-color: #CC0000;
            color: #FFF;
        }
        &.warning:not(:disabled):hover {
            background-color: #FF0000;
        }
        &.warning:not(:disabled):active {
            background-color: #990000;
        }

        &.success {
            background-color: #00CC00;
            color: #FFF;
        }
        &.success:not(:disabled):hover {
            background-color: #00FF00;
        }
        &.success:not(:disabled):active {
            background-color: #009900;
        }

        &.medium, &.small {
            font-style: normal;
            transition: 0.2s all;
        }
        &.medium {
            font-size: 14px;
            padding: 8px 12px;
        }
        &.small {
            font-size: 12px;
            padding: 6px 8px;
        }
    "#
    )
    .unwrap()
}
