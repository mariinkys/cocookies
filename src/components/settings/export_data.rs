use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::{
    components::{dialog::DialogComponent, page_loading::PageLoadingComponent}, utils::settings_utils::get_full_recipes,
};

#[component]
pub fn ExportDataCardComponent() -> impl IntoView {
    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();

    view! {
        <div class="card bg-base-100 shadow-xl w-96 text-center">
            <div class="card-body w-full text-center">
                <p class="text-xl">"Export your data"</p>
                <p class="text-xs">"*Does not export the recipe photos"</p>
                <br/>
                <button class="btn btn-success"
                    on:click=move |_| {
                    let _ = dialog_ref_node.get().unwrap().show_modal();
                }>
                    "Export"
                </button>

                <DialogComponent dialog_title="Export Data" dialog_node_ref=dialog_ref_node dialog_content=move || {
                    let recipes = OnceResource::new(get_full_recipes());

                    view! {
                        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
                            <ErrorBoundary fallback=|error| view! {
                                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
                            }>
                                <Show  when=move || {recipes.get().is_some_and(|res| res.is_ok_and(|recipes| !recipes.is_empty()))}
                                    fallback=move || view! {
                                    <p class="text-center font-bold text-xl text-error">"No recipes found"</p>
                                }>
                                    { move || {
                                        recipes.get().map(move |x| {
                                            x.map(move |recipes_result| {
                                                let recipe_number = recipes_result.len();
                                                
                                                let download_json = move || {
                                                    let json_string = serde_json::to_string(&recipes_result).unwrap_or_default();
                                                    let json_value = wasm_bindgen::JsValue::from_str(&json_string);
                                                    let array = web_sys::js_sys::Array::new();
                                                    array.push(&json_value);
                                                    let blob = web_sys::Blob::new_with_str_sequence(&array).expect("Failed to create Blob");
                                                    let url = web_sys::Url::create_object_url_with_blob(&blob).expect("Failed to create object URL");
                                            
                                                    let document = leptos::prelude::window().document().unwrap();
                                                    let a = document.create_element("a").unwrap();
                                                    a.set_attribute("href", &url).unwrap();
                                                    a.set_attribute("download", "recipes.json").unwrap();
                                                    
                                                    if let Some(a_element) = a.dyn_ref::<web_sys::HtmlElement>() {
                                                        a_element.click(); 
                                                    }

                                                    web_sys::Url::revoke_object_url(&url).expect("Failed to revoke object URL");
                                                };

                                                view! {
                                                    <p>"Export "{recipe_number} " recipes?"</p>
                                                    <br/>
                                                    <button class="btn btn-primary px-8"
                                                        on:click=move |_| download_json()
                                                    >"Export"</button>
                                                }
                                            })
                                        })
                                    }}
                                </Show>
                            </ErrorBoundary>
                        </Suspense>
                    }
                }/>
            </div>
        </div>
    }.into_any()
}
