use crate::api::recipe::add_recipe;
use crate::components::page_loading::PageLoadingComponent;
use crate::components::toast::{ToastMessage, ToastType};
use crate::models::recipe::Recipe;
use crate::utils::{env_utils::EnvOptions, file_utils::upload_file};
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;
use leptos_router::NavigateOptions;

#[component]
pub fn NewRecipe() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let env_options: ReadSignal<EnvOptions> = expect_context();
    let loading = RwSignal::new(false);

    let file_input = NodeRef::<leptos::html::Input>::new();
    let main_photo_image = RwSignal::new(None);
    let image_name = RwSignal::new(String::new());

    let model = RwSignal::new(Recipe::default());
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();
        if !loading.get() {
            let navigate = use_navigate();
            loading.set(true);

            // TODO: How to make loading appear on submit click (why does it wait for a while? file uploading?)
            spawn_local(async move {
                // File upload handling
                let image_bytes = main_photo_image.get_untracked();
                if let Some(img) = image_bytes {
                    let image_path = format!(
                        "{}/{}",
                        env_options.get_untracked().upload_dir,
                        image_name.get_untracked()
                    );

                    if let Err(err) = upload_file(img, image_path).await {
                        set_toast.set(ToastMessage {
                            message: format!("Err {}", err),
                            toast_type: ToastType::Error,
                            visible: true,
                        });
                        loading.set(false);
                        return;
                    } else {
                        let mut updated_model = model.get_untracked();
                        updated_model.main_photo = Some(image_name.get_untracked());
                        model.set(updated_model);
                    }
                }

                // Recipe creation handling
                match add_recipe(model.get_untracked()).await {
                    Ok(_succ) => {
                        set_toast.set(ToastMessage {
                            message: String::from("Success"),
                            toast_type: ToastType::Success,
                            visible: true,
                        });
                        loading.set(false);
                        navigate("/", NavigateOptions::default());
                    }
                    Err(err) => {
                        set_toast.set(ToastMessage {
                            message: format!("Err {}", err),
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
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>
                <Show when=move || !loading.get() fallback=move || view! { <PageLoadingComponent/> }>
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
                                        disabled=loading
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
                                        autocomplete="off"
                                        disabled=loading
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
                                        name="prep_time_minutes"
                                        autocomplete="off"
                                        disabled=loading
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
                                        autocomplete="off"
                                        disabled=loading
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
                                    <input disabled=loading type="file" accept="image/*" class="file-input file-input-bordered w-full" node_ref=file_input on:change=move |_ev| {
                                        if let Some(files) = file_input.get().unwrap().files() {
                                            if let Some(file) = files.get(0) {
                                                let file_type = crate::utils::file_utils::get_file_extension(&file); // Check if it's a valid file extension
                                                if file_type.is_none() {
                                                    set_toast.set(ToastMessage {
                                                        message: String::from("Not an image"),
                                                        toast_type: ToastType::Error,
                                                        visible: true,
                                                    });
                                                    main_photo_image.set(None);
                                                    image_name.set(String::new());
                                                } else {
                                                    spawn_local(async move {
                                                        let promise = file.array_buffer();
                                                        if let Ok(js_value) = wasm_bindgen_futures::JsFuture::from(promise).await {
                                                            let bytes = web_sys::js_sys::Uint8Array::new(&js_value).to_vec();
                                                            main_photo_image.set(Some(bytes));

                                                            image_name.set(format!("{}.{}", uuid::Uuid::new_v4(), file_type.unwrap()))
                                                        } else {
                                                            main_photo_image.set(None);
                                                            image_name.set(String::new());
                                                        }
                                                    });
                                                }
                                            }
                                        }
                                    }/>
                                </div>

                                <button disabled=loading class="btn btn-primary mt-3 w-full" type="submit">"Add"</button>
                            </form>
                        </div>
                    </div>
                </Show>
            </ErrorBoundary>
        </Suspense>
    }
}
