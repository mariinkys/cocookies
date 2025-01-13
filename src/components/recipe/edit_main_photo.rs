use leptos::{prelude::*, task::spawn_local};

use crate::{
    api::recipe::update_recipe_main_photo,
    components::toast::{ToastMessage, ToastType},
    utils::{upload_file, EnvOptions},
};

#[component]
pub fn EditMainPhotoComponent(recipe_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let env_options: ReadSignal<EnvOptions> = expect_context();
    let loading = RwSignal::new(false);

    let file_input = NodeRef::<leptos::html::Input>::new();
    let main_photo_image = RwSignal::new(None);
    let image_name = RwSignal::new(String::new());
    let on_image_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        if !loading.get()
            && main_photo_image.get_untracked().is_some()
            && !image_name.get_untracked().is_empty()
        {
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
                    }
                }

                // Recipe main photo update handling
                match update_recipe_main_photo(recipe_id, image_name.get_untracked()).await {
                    Ok(_succ) => {
                        set_toast.set(ToastMessage {
                            message: String::from("Success"),
                            toast_type: ToastType::Success,
                            visible: true,
                        });
                        loading.set(false);
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
        <Show when=move || !loading.get() fallback=move || {
            view! {
                <p class="text-center font-bold text-xl">"Uploading Photo"</p>
                <p class="text-center font-bold text-xl text-error">"DO NOT CLOSE"</p>
            }
        }>
            <form class="flex flex-col gap-3" on:submit=on_image_submit>
                <div>
                    <div class="label p-0">
                        <span class="label-text">"Main Photo"</span>
                    </div>
                    <input disabled=loading type="file" accept="image/*" class="file-input file-input-bordered w-full" node_ref=file_input on:change=move |_ev| {
                        if let Some(files) = file_input.get().unwrap().files() {
                            if let Some(file) = files.get(0) {
                                let file_type = crate::utils::get_file_extension(&file); // Check if it's a valid file extension
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

                <button disabled=loading class="btn btn-primary mt-3 w-full" type="submit">"Update"</button>
            </form>
        </Show>
    }
}
