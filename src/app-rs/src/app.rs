use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[function_component(App)]
pub fn app() -> Html {
    let answer_msg = use_state(|| String::new());
    let answer_msg_rc = Rc::new(answer_msg.clone());

    let answer = {
        let answer_msg_rc = answer_msg_rc.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let answer_msg = answer_msg_rc.clone();

            spawn_local(async move {
                let new_msg = invoke("answer", JsValue::UNDEFINED)
                    .await
                    .as_string()
                    .unwrap();
                answer_msg.set(new_msg);
            });
        })
    };

    html! {
        <main class="container">
            <div class="row">
                <img src="public/deep_thought.png" class="logo tauri" alt="deep_thought"/>
            </div>

            <form class="row" onclick={answer}>
                <button type="button">{"Give me the answer to life, universe and everything"}</button>
            </form>

            <p><b>{ &*answer_msg }</b></p>
        </main>
    }
}
