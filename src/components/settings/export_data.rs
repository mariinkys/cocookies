use leptos::prelude::*;

#[component]
pub fn ExportDataCardComponent() -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow-xl w-96 text-center">
            <div class="card-body w-full text-center">
                <p class="text-xl">"Export your data"</p>
                <p class="text-xs">"*Does not export the recipe photos"</p>
                <br/>
                <button class="btn btn-success">"Export"</button>
            </div>
        </div>
    }
}
