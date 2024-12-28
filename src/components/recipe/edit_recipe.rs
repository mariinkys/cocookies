use leptos::prelude::*;

use crate::{api::recipe::UpdateRecipe, models::recipe::Recipe};

#[component]
pub fn ViewEditRecipeComponent(recipe: Recipe) -> impl IntoView {
    let model = RwSignal::new(recipe);
    let update_recipe = ServerAction::<UpdateRecipe>::new();

    // TODO: Error Handling && User Feedback when Changes Saved

    view! {
        <div class="w-full card shadow-xl">
            <div class="card-body">
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
            </div>
        </div>
    }
}
