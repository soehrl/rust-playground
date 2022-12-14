use leptos::*;
use leptos_meta::*;
// use wasm_bindgen::prelude::*; 

// use wasm_bindgen::

#[component]
pub fn App(cx: Scope) -> Element {
    provide_context(cx, MetaContext::default());
    
    let (value, set_value) = create_signal(cx, cx.id().to_json().unwrap());
    create_effect(cx, move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        let context = canvas.get_context("webgl2").unwrap().expect("foo").dyn_into::<web_sys::WebGl2RenderingContext>();

        if context.is_ok() {
            console_log("Context created!");
        }

        // let c = can
        // let context = canvas.get_context("webgl2")?.unwrap().dyn_into::<web_sys::WebGl2RenderingContext>().unwrap();
    });

    view! {
        cx,
        <div>
            <canvas id=value></canvas>
        </div>
    }
}
