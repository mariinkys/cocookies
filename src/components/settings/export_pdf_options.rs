use leptos::prelude::*;

use crate::{
    api::config::{get_config, UpdateConfig},
    components::{page_loading::PageLoadingComponent, toast::{ToastMessage, ToastType}},
};

#[component]
pub fn ExportPDFOptionsComponent() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let config = OnceResource::new(get_config());
    let update_config = ServerAction::<UpdateConfig>::new();

    // 'value' holds the latest *returned* value from the server
    let value = update_config.value();
    Effect::new(move |_| {
        if let Some(val) = value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Changed Saved"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Error Saving {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });
    
    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{error:?}")}</p>
            }>
            { move || {
                config.get().map(move |c| {
                    c.map(move |config| {
                        let model = RwSignal::new(config);

                        view! {
                            <ActionForm action=update_config>
                                <div class="card bg-base-100 shadow-xl w-96">
                                    <div class="card-body w-full">
                                        <a class="link" target="_blank" href="https://github.com/gotenberg/gotenberg" class="text-xl">"Set Gotenberg Server Location"</a>
                                        <p class="text-xs">"*Ej: http://localhost:3000"</p>
                                        <br/>
                                        <fieldset>
                                            <input type="text"
                                                class="input w-full"
                                                name="gotenberg_location"
                                                id="gotenberg_location"
                                                autocomplete="off"
                                                prop:value=move || model.get().gotenberg_location.unwrap_or_default()
                                                on:input=move |ev| {
                                                    if !event_target_value(&ev).is_empty() {
                                                        model.update(|curr| {
                                                            curr.gotenberg_location = Some(event_target_value(&ev));
                                                        });
                                                    } else {
                                                        model.update(|curr| {
                                                            curr.gotenberg_location = None;
                                                        });
                                                    }
                                                }
                                            />
                                        </fieldset>
                                        <br/>
                                        <div class="flex flex-row-reverse gap-1">
                                            <button class="btn btn-success"
                                                on:click=move |_| {
                                                
                                            }>
                                                "Save"
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </ActionForm>
                        }
                    })
                })
            }}
            </ErrorBoundary>
        </Suspense>
    }.into_any()
}
