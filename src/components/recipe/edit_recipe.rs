use leptos::prelude::*;

use crate::{
    api::recipe::UpdateRecipe,
    components::{
        dialog::DialogComponent,
        recipe::edit_main_photo::EditMainPhotoComponent,
        toast::{ToastMessage, ToastType},
    },
    models::recipe::Recipe, utils::env_utils::EnvOptions,
};

#[component]
pub fn ViewEditRecipeComponent(recipe: Recipe) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let env_options: ReadSignal<EnvOptions> = expect_context();
    let model = RwSignal::new(recipe);
    let update_recipe = ServerAction::<UpdateRecipe>::new();

    // 'value' holds the latest *returned* value from the server
    let value = update_recipe.value();
    Effect::new(move |_| {
        if let Some(val) = value() {
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
                        message: format!("Error Saving {}", err),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });

    let edit_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

    let change_main_photo_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let photo_path = if model.get().main_photo.is_some() {
        format!("..{}/{}", env_options.get().upload_dir, model.read_only().get().main_photo.unwrap_or_default())
    } else {
        String::from("/assets/utils/image-not-found.png")
    };

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
                                <EditMainPhotoComponent recipe_id=recipe_id/>
                            }
                        }/>
                    </div>
                    <div class="flex flex-col justify-between w-full">
                        <div class="w-full">
                            <div class="flex justify-between items-center w-full">
                                <h1 class="text-4xl font-bold">{move || model.read_only().get().name}</h1>
                                <button
                                    class="btn btn-sm btn-ghost"
                                    on:click=move |_| {
                                        let _ = edit_dialog_ref.get().unwrap().show_modal();
                                    }
                                >
                                    "Edit"
                                </button>
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
                            <div class="label p-0">
                                <span class="label-text">"Recipe Name"</span>
                            </div>
                            <input type="text"
                                class="input input-bordered w-full"
                                name="name"
                                required
                                autocomplete="off"
                                prop:value={move || model.get().name}
                                on:input=move |ev| {
                                    model.update(|curr| {
                                        curr.name = event_target_value(&ev);
                                    });
                                }
                            />

                            // Recipe: description
                            <div class="label p-0">
                                <span class="label-text">"Description"</span>
                            </div>
                            <input type="text"
                                class="input input-bordered w-full"
                                name="description"
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

                            // Recipe: prep_time
                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Preparation Time (minutes)"</span>
                                </div>
                                <input type="number"
                                    class="input input-bordered w-full"
                                    name="prep_time_minutes"
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

                            // Recipe: servings
                            <div>
                                <div class="label p-0">
                                    <span class="label-text">"Servings"</span>
                                </div>
                                <input type="number"
                                    class="input input-bordered w-full"
                                    name="servings"
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

                            <button type="submit" class="btn btn-primary w-full mt-2">"Save"</button>
                        </ActionForm>
                    }
                }/>
            </div>
        </div>
    }.into_any()
}
