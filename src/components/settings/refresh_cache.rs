// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "reload", js_namespace = ["window", "location"])]
    fn force_reload(force_get: bool);
}

#[component]
pub fn RefreshCacheComponent() -> impl IntoView {
    let hard_refresh = move |_| {
        force_reload(true);
    };

    view! {
        <div class="card bg-base-100 shadow-xl w-96">
            <div class="card-body w-full h-full justify-between">
                <p class="text-xl text-center">"Refresh Application Cache"</p>
                <button class="btn btn-success w-full"
                    on:click=hard_refresh>
                    "Refresh"
                </button>
            </div>
        </div>
    }
    .into_any()
}
