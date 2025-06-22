// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

#[component]
pub fn PageLoadingComponent() -> impl IntoView {
    view! {
        <p class="text-center font-bold text-xl">"Loading..."</p>
    }
}
