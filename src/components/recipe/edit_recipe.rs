// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;

use crate::{
    api::{config::export_pdf, recipe::UpdateRecipe},
    components::{
        dialog::DialogComponent,
        recipe::edit_main_photo::EditMainPhotoComponent,
        toast::{ToastMessage, ToastType},
    },
    models::recipe::Recipe, utils::{env_utils::EnvOptions, pdf_themes::PdfTheme},
};

#[component]
pub fn ViewEditRecipeComponent(recipe: Recipe, main_photo_change: WriteSignal<bool>) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let env_options: ReadSignal<EnvOptions> = expect_context();
    let model = RwSignal::new(recipe);
    let update_recipe = ServerAction::<UpdateRecipe>::new();

    // 'value' holds the latest *returned* value from the server
    let value = update_recipe.value();
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

    let edit_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    let change_main_photo_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let photo_path = if model.get_untracked().main_photo.is_some() {
        let upload_dir = env_options.get_untracked().upload_dir;
        if upload_dir.starts_with('/') {
            format!("{}/{}", upload_dir, model.read_only().get_untracked().main_photo.unwrap_or_default())
        }else{
            format!("/{}/{}", upload_dir, model.read_only().get_untracked().main_photo.unwrap_or_default())
        }
    } else {
        String::from("/assets/utils/image-not-found.png")
    };

    let export_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let export_pdf_action = Action::new(move |(recipe_id, selected_theme): &(i32, Option<PdfTheme>)| {
        let recipe_id = *recipe_id;
        let selected_theme = selected_theme.clone();
        async move { 
            let result = export_pdf(recipe_id, selected_theme).await;
            match result {
                Ok(base64_pdf) => {
                    let download_pdf = move || {
                        let window = leptos::prelude::window();
                        let document = window.document().unwrap();
                        
                        // Decode base64
                        let binary_string = window.atob(&base64_pdf).unwrap();
                        let bytes = web_sys::js_sys::Uint8Array::new_with_length(binary_string.len() as u32);
                        
                        for (i, char) in binary_string.chars().enumerate() {
                            bytes.set_index(i as u32, char as u8);
                        }
                        
                        let array = web_sys::js_sys::Array::new();
                        array.push(&bytes);
                        let blob = web_sys::Blob::new_with_u8_array_sequence(&array).unwrap();
                        
                        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                        
                        let a = document.create_element("a").unwrap();
                        a.set_attribute("href", &url).unwrap();
                        a.set_attribute("download", "export.pdf").unwrap();
                        
                        if let Some(a_element) = wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&a) {
                            document.body().unwrap().append_child(&a).unwrap();
                            a_element.click();
                            document.body().unwrap().remove_child(&a).unwrap();
                            
                            // Clean up immediately after click
                            web_sys::Url::revoke_object_url(&url).unwrap();
                        } else {
                            // Clean up if casting failed
                            web_sys::Url::revoke_object_url(&url).unwrap();
                        }
                    };
    
                    download_pdf();
                },
                Err(err) => {
                     set_toast.set(ToastMessage {
                        message: format!("Error Exporting {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                },
            }
        }
    });

    view! {
        <div class="w-full card shadow-xl">
            <div class="card-body">
                <div class="flex flex-wrap md:flex-nowrap gap-3">
                    <div class="flex-none relative w-48 h-48">
                        <img class="w-full h-full object-cover shadow-inner rounded-full" src=photo_path/>
                        <button
                            class="absolute inset-0 w-full h-full bg-transparent rounded-full"
                            on:click=move |_| {
                                let _ = change_main_photo_dialog_ref.get().unwrap().show_modal();
                            }
                        >
                            <span class="sr-only">"Click to open change image dialog"</span>
                        </button>
                        <DialogComponent dialog_title="Edit Main Photo" dialog_node_ref=change_main_photo_dialog_ref dialog_content=move || {
                            let recipe_id = model.get_untracked().recipe_id.unwrap_or_default();
                            
                            view! {
                                <EditMainPhotoComponent recipe_id=recipe_id updated_toggle=main_photo_change/>
                            }
                        }/>
                    </div>
                    <div class="flex flex-col justify-between w-full">
                        <div class="w-full">
                            <div class="flex justify-between items-center w-full">
                                <h1 class="text-4xl font-bold">{move || model.read_only().get().name}</h1>
                                <div class="flex gap-1">
                                    <button
                                        class="btn btn-sm btn-ghost"
                                        on:click=move |_| {
                                            let _ = edit_dialog_ref.get().unwrap().show_modal();
                                        }
                                    >
                                        "Edit"
                                    </button>

                                    <button
                                        class="btn btn-sm btn-secondary text-black"
                                        on:click=move |_| {
                                            let _ = export_dialog_ref.get().unwrap().show_modal();
                                        }
                                    >
                                        "Export PDF"
                                    </button>
                                    <DialogComponent dialog_title="Export PDF" dialog_node_ref=export_dialog_ref dialog_content=move || {
                                        let recipe_id = model.get_untracked().recipe_id.unwrap_or_default();
                                        let selected_theme = RwSignal::new(PdfTheme::default());
                                        
                                        view! {
                                            <div class="flex flex-col gap-2 w-full">
                                                <div class="w-full">
                                                    <fieldset class="fieldset">
                                                        <label class="label" for="theme">"Theme"</label>
                                                        <select
                                                            class="select select-primary w-full"
                                                            id="theme"
                                                            on:change=move |ev| {
                                                                let new_theme_str = event_target_value(&ev);
                                                                if let Ok(new_theme) = new_theme_str.parse::<PdfTheme>() {
                                                                    selected_theme.set(new_theme);
                                                                }
                                                            }
                                                        >
                                                            <For
                                                                each=move || PdfTheme::ALL.iter()
                                                                key=|theme| *theme
                                                                children=move |theme| {
                                                                    let is_selected = move || selected_theme.get() == *theme;
                                                                    
                                                                    view! {
                                                                        <option 
                                                                            value=theme.to_string()
                                                                            selected=is_selected
                                                                        >
                                                                            {theme.to_string()}
                                                                        </option>
                                                                    }
                                                                }
                                                            />
                                                        </select>
                                                    </fieldset>
                                                </div>

                                                <button
                                                    class="btn btn-primary w-full" 
                                                    on:click=move |_| {
                                                        export_pdf_action.dispatch((recipe_id, Some(selected_theme.get_untracked())));
                                                    }
                                                >
                                                    "Export"
                                                </button>
                                            </div>
                                        }
                                    }/>
                                </div>
                            </div>
                            <p>{move || model.read_only().get().description}</p>
                        </div>
                        <div class="flex gap-3 mt-3 md:mt-0">
                            <Show
                                when=move || { model.read_only().get().servings.is_some() }
                                fallback=|| view! { "" }
                            >
                                <div class="badge badge-primary font-bold">{move || model.read_only().get().servings.unwrap_or_default()}" servings"</div>
                            </Show>
                            <Show
                                when=move || { model.read_only().get().prep_time_minutes.is_some() }
                                fallback=|| view! { "" }
                            >
                                <div class="badge badge-secondary font-bold">{move || model.read_only().get().prep_time_minutes.unwrap_or_default()}"min"</div>
                            </Show>
                        </div>
                    </div>
                </div>

                <DialogComponent dialog_title="Edit Recipe" dialog_node_ref=edit_dialog_ref dialog_content=move || {
                    view! {
                        <ActionForm action=update_recipe>
                            // We need the id for the update but we don't want to show it.
                            <input type="hidden" name="recipe_id" autocomplete="off" prop:value={move || model.get().recipe_id.unwrap_or_default()}/>

                            // Recipe: name
                            <fieldset class="fieldset">
                                <label class="label" for="name">"Recipe Name"</label>
                                <input type="text"
                                    class="input w-full"
                                    name="name"
                                    id="name"
                                    required
                                    autocomplete="off"
                                    prop:value={move || model.get().name}
                                    on:input=move |ev| {
                                        model.update(|curr| {
                                            curr.name = event_target_value(&ev);
                                        });
                                    }
                                />
                            </fieldset>

                            // Recipe: description
                            <fieldset>
                                <label class="label" for="description">"Description"</label>
                                <input type="text"
                                    class="input w-full"
                                    name="description"
                                    id="description"
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
                            </fieldset>

                            // Recipe: prep_time
                            <div>
                                <fieldset>
                                    <label class="label" for="prep_time_minutes">"Preparation Time (minutes)"</label>
                                    <input type="number"
                                        class="input w-full"
                                        name="prep_time_minutes"
                                        id="prep_time_minutes"
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
                                </fieldset>
                            </div>

                            // Recipe: servings
                            <div>
                                <fieldset>
                                    <label class="label" for="servings">"Servings"</label>
                                    <input type="number"
                                        class="input w-full"
                                        name="servings"
                                        id="servings"
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
                                </fieldset>
                            </div>

                            <button type="submit" class="btn btn-primary w-full mt-2">"Save"</button>
                        </ActionForm>
                    }
                }/>
            </div>
        </div>
    }.into_any()
}
