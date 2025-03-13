use leptos::{prelude::*, task::spawn_local};

use crate::{
    components::{dialog::DialogComponent, page_loading::PageLoadingComponent, toast::{ToastMessage, ToastType}}, utils::settings_utils::{insert_full_recipes, FullRecipe},
};

#[component]
pub fn ImportDataCardComponent() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let file_input = NodeRef::<leptos::html::Input>::new();
    let loading = RwSignal::new(false);

    let file_bytes= RwSignal::new(None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        if !loading.get() && file_bytes.get_untracked().is_some()
        {
            loading.set(true);
            spawn_local(async move {
                let bytes: Vec<u8> = file_bytes.get_untracked().unwrap();

                match serde_json::from_slice::<Vec<FullRecipe>>(&bytes) {
                    Ok(full_recipes) => {
                        match insert_full_recipes(full_recipes).await {
                            Ok(_) => {
                                set_toast.set(ToastMessage {
                                    message: String::from("Imported Correctly!"),
                                    toast_type: ToastType::Success,
                                    visible: true,
                                });
                                loading.set(false);
                            },
                            Err(e) => {
                                set_toast.set(ToastMessage {
                                    message: format!("Error Adding to DB: {}", e),
                                    toast_type: ToastType::Error,
                                    visible: true,
                                });
                                loading.set(false);
                            },
                        }
                    }
                    Err(_e) => {
                        set_toast.set(ToastMessage {
                            message: String::from("Error Converting to Recipes"),
                            toast_type: ToastType::Error,
                            visible: true,
                        });
                        loading.set(false);
                    }
                }
            });
        }
    };

    view! {
        <div class="card bg-base-100 shadow-xl w-96 text-center">
            <div class="card-body w-full text-center">
                <p class="text-xl">"Import data"</p>
                <br/>
                <br/>
                <button class="btn btn-primary"
                    on:click=move |_| {
                    let _ = dialog_ref_node.get().unwrap().show_modal();
                }>
                    "Import"
                </button>

                <DialogComponent dialog_title="Import Data" dialog_node_ref=dialog_ref_node dialog_content=move || {
                    view! {
                        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
                            <ErrorBoundary fallback=|error| view! {
                                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
                            }>
                                <form class="flex flex-col gap-3" on:submit=on_submit>
                                    <div>
                                        <div class="label p-0">
                                            <span class="label-text">"File to Import"</span>
                                        </div>
                                        <input disabled=loading type="file" accept=".json" class="file-input w-full" node_ref=file_input on:change=move |_ev| {
                                            if let Some(files) = file_input.get().unwrap().files() {
                                                if let Some(file) = files.get(0) {
                                                    if file.type_().as_str() != "application/json" {
                                                        set_toast.set(ToastMessage {
                                                            message: String::from("Not JSON"),
                                                            toast_type: ToastType::Error,
                                                            visible: true,
                                                        });
                                                        file_bytes.set(None);
                                                    } else {
                                                        spawn_local(async move {
                                                            let promise = file.array_buffer();
                                                            if let Ok(js_value) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                                                let bytes = web_sys::js_sys::Uint8Array::new(&js_value).to_vec();
                                                                file_bytes.set(Some(bytes));
                                                            } else {
                                                                set_toast.set(ToastMessage {
                                                                    message: String::from("Error Uploading"),
                                                                    toast_type: ToastType::Error,
                                                                    visible: true,
                                                                });
                                                                file_bytes.set(None);
                                                            }
                                                        });
                                                    }
                                                }
                                            }
                                        }/>
                                    </div>
                    
                                    <button disabled=loading class="btn btn-primary mt-3 w-full" type="submit">"Update"</button>
                                </form>
                            </ErrorBoundary>
                        </Suspense>
                    }
                }/>
            </div>
        </div>
    }.into_any()
}
