use leptos::prelude::*;

#[component]
pub fn ImportDataCardComponent() -> impl IntoView {
    view! {
        <div class="card bg-base-100 shadow-xl w-96 text-center">
            <div class="card-body w-full text-center">
                <p class="text-xl">"Import data"</p>
                <br/>
                <br/>
                <button class="btn btn-primary">"Import"</button>
            </div>
        </div>
    }
}
