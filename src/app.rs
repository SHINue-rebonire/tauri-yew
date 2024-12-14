use crate::components::atoms::button::button::Button;
use crate::components::atoms::button::button::{ButtonSize, ButtonVariant};

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[derive(Serialize, Deserialize)]
struct ButtonLabelArgs<'a> {
    label: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();

    let name = use_state(|| String::new());

    let greet_msg = use_state(|| String::new());
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with(name2, move |_| {
            spawn_local(async move {
                if name.is_empty() {
                    return;
                }

                let args = serde_wasm_bindgen::to_value(&GreetArgs { name: &*name }).unwrap();
                // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
                let new_msg = invoke("greet", args).await.as_string().unwrap();
                greet_msg.set(new_msg);
            });

            || {}
        });
    }

    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    let button_msg = use_state(|| String::new());
    let button_label = use_state(|| String::new());
    {
        let button_msg = button_msg.clone();
        let button_label = button_label.clone();

        use_effect_with(button_label.clone(), move |label| {
            let label_value = label.clone();

            spawn_local(async move {
                if label_value.is_empty() {
                    return;
                }

                let args = serde_wasm_bindgen::to_value(&ButtonLabelArgs {
                    label: &*label_value,
                })
                .unwrap();

                let js_value = invoke("button_label", args).await.as_string().unwrap();
                button_msg.set(js_value);
            });

            || {}
        });
    }

    let on_click = {
        let button_label = button_label.clone();
        move |label: &'static str| {
            let button_label = button_label.clone();
            Callback::from(move |_| {
                button_label.set(label.to_string());
            })
        }
    };

    html! {
            <main class="container">
                <h1>{"Welcome to Tauri + Yew"}</h1>

                <div class="row">
                    <a href="https://tauri.app" target="_blank">
                        <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                    </a>
                    <a href="https://yew.rs" target="_blank">
                        <img src="public/yew.png" class="logo yew" alt="Yew logo"/>
                    </a>
                </div>
                <p>{"Click on the Tauri and Yew logos to learn more."}</p>

                <form class="row" onsubmit={greet}>
                    <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                    <button type="submit">{"Greet"}</button>
                </form>
                <p>{ &*greet_msg }</p>

                <div class="btn-row">
                    <Button
                        label="Primary"
                        on_click={on_click("Primary")}
                        disabled={false}
                        size={ButtonSize::Medium}
                        variant={ButtonVariant::Primary}
                    />
                    <Button
                        label="Secondary"
                        on_click={on_click("Secondary")}
                        disabled={false}
                        size={ButtonSize::Medium}
                        variant={ButtonVariant::Secondary}
                    />
                    <Button
                        label="Warning"
                        on_click={on_click("Warning")}
                        disabled={false}
                        size={ButtonSize::Medium}
                        variant={ButtonVariant::Warning}
                    />
                    <Button
                        label="Success"
                        on_click={on_click("Success")}
                        disabled={false}
                        size={ButtonSize::Medium}
                        variant={ButtonVariant::Success}
                    />
                </div>
                <p>{ &*button_msg }</p>
            </main>
    }
}
