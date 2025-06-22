// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::{
    api::config::{TestConfig},
    components::{page_loading::PageLoadingComponent, toast::{ToastMessage, ToastType}},
};

#[component]
pub fn TestPDFOptionsComponent() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let test_config = ServerAction::<TestConfig>::new();

    // 'value' holds the latest *returned* value from the server
    let value = test_config.value();
    Effect::new(move |_| {
        if let Some(val) = value.get() {
            match val {
                Ok(_) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Gotenberg is Ok!"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("{err}"),
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
                <ActionForm action=test_config>
                    <div class="card bg-base-100 shadow-xl w-96">
                        <div class="card-body w-full">
                            <p class="text-xl">"Test Gotenberg Status"</p>
                            <div class="flex flex-row-reverse gap-1">
                                <button class="btn btn-success w-full"
                                    on:click=move |_| {
                                    
                                }>
                                    "Test"
                                </button>
                            </div>
                        </div>
                    </div>
                </ActionForm>
            </ErrorBoundary>
        </Suspense>
    }.into_any()
}
