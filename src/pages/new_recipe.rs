use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::api::recipe::upsert_recipe;
use crate::components::page_loading::PageLoadingComponent;
use crate::components::toast::{ToastMessage, ToastType};
use crate::models::recipe::Recipe;

#[component]
pub fn NewRecipe() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let model = RwSignal::new(Recipe::default());
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        let navigate = use_navigate();

        // TODO: Data validator

        // TODO: File upload handling

        spawn_local(async move {
            match upsert_recipe(model.get_untracked()).await {
                Ok(_succ) => {
                    set_toast.set(ToastMessage {
                        message: String::from("Success"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    navigate("/", Default::default());
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Err {}", err),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        });
    };

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>
                <div class="mx-auto max-w-lg card shadow-xl">
                    <div class="card-body">
                        <h1 class="text-3xl font-bold text-center">"New Recipe"</h1>

                        <form class="flex flex-col gap-3" on:submit=on_submit>
                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Name"</span>
                                </div>
                                <input type="text"
                                    class="input input-bordered w-full"
                                    name="name"
                                    required
                                    autocomplete="off"
                                    prop:value={move || model.get().name }
                                    on:input=move |ev| {
                                        model.update(|curr| {
                                                curr.name = event_target_value(&ev);
                                        });
                                    }
                                />
                            </div>

                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Description"</span>
                                </div>
                                <input type="text"
                                    class="input input-bordered w-full"
                                    name="description"
                                    required
                                    autocomplete="off"
                                    prop:value=move || model.get().description.unwrap_or_default()
                                    on:input=move |ev| {
                                        if !event_target_value(&ev).is_empty() {
                                            model.update(|curr| {
                                                curr.description = Some(event_target_value(&ev));
                                            });
                                        } else {
                                            model.update(|curr| {
                                                curr.description = None;
                                            });
                                        }
                                    }
                                />
                            </div>

                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Preparation Time (minutes)"</span>
                                </div>
                                <input type="number"
                                    class="input input-bordered w-full"
                                    name="prep_time"
                                    required
                                    autocomplete="off"
                                    prop:value={move || model.get().prep_time_minutes }
                                    on:input=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                            model.update(|curr| {
                                                curr.prep_time_minutes = Some(value);
                                            });
                                        } else {
                                            model.update(|curr| {
                                                curr.prep_time_minutes = None;
                                            });
                                        }

                                    }
                                />
                            </div>

                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Servings"</span>
                                </div>
                                <input type="number"
                                    class="input input-bordered w-full"
                                    name="servings"
                                    required
                                    autocomplete="off"
                                    prop:value={move || model.get().servings }
                                    on:input=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                            model.update(|curr| {
                                                curr.servings = Some(value);
                                            });
                                        } else {
                                            model.update(|curr| {
                                                curr.servings = None;
                                            });
                                        }

                                    }
                                />
                            </div>

                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Main Photo"</span>
                                </div>
                                <input type="file" class="file-input file-input-bordered w-full" />
                            </div>

                            <button class="btn btn-primary mt-3 w-full" type="submit">"Add"</button>
                        </form>
                    </div>
                </div>
            </ErrorBoundary>
        </Suspense>
    }
}
