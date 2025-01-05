use leptos::prelude::*;

use crate::{
    api::recipe_ingredients::{
        get_all_recipe_ingredients, AddRecipeIngredients, DeleteRecipeIngredient,
        UpdateRecipeIngredients,
    },
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    models::recipe_ingredient::RecipeIngredient,
};

#[component]
pub fn ViewEditIngredientsComponent(recipe_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let new_ingredient_model = RwSignal::new(RecipeIngredient::init(recipe_id));
    let recipe_ingredients_resource = Resource::new(
        move || recipe_id,
        move |recipe_id| async move { get_all_recipe_ingredients(recipe_id).await },
    );

    let update_recipe_ingredient = ServerAction::<UpdateRecipeIngredients>::new();
    let update_value = update_recipe_ingredient.value();
    Effect::new(move |_| {
        if let Some(val) = update_value() {
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

    let add_dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();
    let add_recipe_ingredient = ServerAction::<AddRecipeIngredients>::new();
    let add_value = add_recipe_ingredient.value();
    Effect::new(move |_| {
        if let Some(val) = add_value() {
            match val {
                Ok(_) => {
                    new_ingredient_model.set(RecipeIngredient::init(recipe_id));
                    recipe_ingredients_resource.refetch();
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

    let delete_recipe_ingredient = ServerAction::<DeleteRecipeIngredient>::new();
    let delete_value = delete_recipe_ingredient.value();
    Effect::new(move |_| {
        if let Some(val) = delete_value() {
            match val {
                Ok(_) => {
                    recipe_ingredients_resource.refetch();
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

    view! {
        <div class="w-full card shadow-xl">
            <div class="card-body">
                <div class="flex justify-between items-center">
                    <h1 class="text-4xl font-bold">"Ingredients"</h1>
                    <button
                        class="btn btn-sm btn-primary"
                        on:click=move |_| {
                            let _ = add_dialog_ref_node.get().unwrap().show_modal();
                        }
                    >
                        "Add New"
                    </button>
                </div>
                <Suspense fallback= move || view! { <PageLoadingComponent/> }>
                    <ErrorBoundary fallback=|error| view! {
                        <p class="text-3xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
                    }>
                        { move || {
                            recipe_ingredients_resource.get().map(move |x| {
                                x.map(move |recipe_ingredient_result| {
                                    let recipe_ingredients = RwSignal::new(recipe_ingredient_result);
                                    view! {
                                        <For each=move || recipe_ingredients.get() key=|ingredient| ingredient.recipe_ingredient_id children=move |recipe_ingredient| {
                                            let model = RwSignal::new(recipe_ingredient);
                                            let update_dialog_ref: NodeRef<leptos::html::Dialog> = NodeRef::new();

                                            view! {
                                                <div class="flex justify-between items-center gap-3">
                                                    <button
                                                        class="btn btn-sm btn-ghost"
                                                        on:click=move |_| {
                                                            let _ = update_dialog_ref.get().unwrap().show_modal();
                                                        }
                                                    >
                                                        "Edit"
                                                    </button>
                                                    <p>{move || model.read_only().get().quantity}" "{move || model.read_only().get().unit}" - "{move || model.read_only().get().ingredient_name}</p>
                                                </div>

                                                <DialogComponent dialog_title="Edit Ingredient" dialog_node_ref=update_dialog_ref dialog_content=move || view! {
                                                    <ActionForm action=update_recipe_ingredient>
                                                        <div class="flex flex-wrap gap-2 items-center">
                                                            // We need the id for the update but we don't want to show it.
                                                            <input type="hidden" name="recipe_ingredient_id" autocomplete="off" prop:value={move || model.get().recipe_ingredient_id.unwrap_or_default()}/>

                                                            // Recipe Ingredient: ingredient_name
                                                            <div class="flex-1 min-w-[200px]">
                                                                <div class="label p-0">
                                                                    <span class="label-text">"Ingredient Name"</span>
                                                                </div>
                                                                <input type="text"
                                                                    class="input input-bordered w-full"
                                                                    name="ingredient_name"
                                                                    required
                                                                    autocomplete="off"
                                                                    prop:value={move || model.get().ingredient_name}
                                                                    on:input=move |ev| {
                                                                        model.update(|curr| {
                                                                            curr.ingredient_name = event_target_value(&ev);
                                                                        });
                                                                    }
                                                                />
                                                            </div>

                                                            // Recipe Ingredient: quantity
                                                            // TODO: Can't input a decimal number on this input
                                                            <div class="flex-1 min-w-[200px]">
                                                                <div class="label p-0">
                                                                    <span class="label-text">"Quantity"</span>
                                                                </div>
                                                                <input type="text"
                                                                    class="input input-bordered w-full"
                                                                    name="quantity"
                                                                    required
                                                                    autocomplete="off"
                                                                    prop:value={move || model.get().quantity}
                                                                    on:input=move |ev| {
                                                                        if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                                                            model.update(|curr| {
                                                                                curr.quantity = value;
                                                                            });
                                                                        } else {
                                                                            model.update(|curr| {
                                                                                curr.quantity = 0.0;
                                                                            });
                                                                        }
                                                                    }
                                                                />
                                                            </div>

                                                            // Recipe: unit
                                                            <div class="flex-1 min-w-[200px]">
                                                                <div class="label p-0">
                                                                    <span class="label-text">"Unit"</span>
                                                                </div>
                                                                <input type="text"
                                                                    class="input input-bordered w-full"
                                                                    name="unit"
                                                                    autocomplete="off"
                                                                    prop:value=move || model.get().unit.unwrap_or_default()
                                                                    on:input=move |ev| {
                                                                        if !event_target_value(&ev).is_empty() {
                                                                            model.update(|curr| {
                                                                                curr.unit = Some(event_target_value(&ev));
                                                                            });
                                                                        } else {
                                                                            model.update(|curr| {
                                                                                curr.unit = None;
                                                                            });
                                                                        }
                                                                    }
                                                                />
                                                            </div>

                                                            <div class="sm:w-min w-full">
                                                                <ActionForm action=delete_recipe_ingredient>
                                                                    // We need the id for the update but we don't want to show it.
                                                                    <input type="hidden" name="recipe_ingredient_id" autocomplete="off" prop:value={move || model.get().recipe_ingredient_id.unwrap_or_default()}/>

                                                                    <button type="submit" class="btn btn-warning mt-[20px] sm:w-min w-full">"Delete"</button>
                                                                </ActionForm>
                                                            </div>

                                                            <button type="submit" class="btn btn-primary mt-[20px] sm:w-min w-full">"Update"</button>
                                                        </div>
                                                    </ActionForm>
                                                }/>
                                            }
                                        }/>


                                    }
                                })
                            })
                        }}
                    </ErrorBoundary>
                </Suspense>

                <DialogComponent dialog_title="Add Ingredient" dialog_node_ref=add_dialog_ref_node dialog_content=move || {
                    view! {
                        <ActionForm action=add_recipe_ingredient>
                            <div class="flex flex-wrap gap-2 items-center">
                                // We need the id for the update but we don't want to show it.
                                <input type="hidden" name="recipe_id" autocomplete="off" prop:value={move || new_ingredient_model.get().recipe_id}/>

                                // Recipe Ingredient: ingredient_name
                                <div class="flex-1 min-w-[200px]">
                                    <div class="label p-0">
                                        <span class="label-text">"Ingredient Name"</span>
                                    </div>
                                    <input type="text"
                                        class="input input-bordered w-full"
                                        name="ingredient_name"
                                        required
                                        autocomplete="off"
                                        prop:value={move || new_ingredient_model.get().ingredient_name}
                                        on:input=move |ev| {
                                            new_ingredient_model.update(|curr| {
                                                curr.ingredient_name = event_target_value(&ev);
                                            });
                                        }
                                    />
                                </div>

                                // Recipe Ingredient: quantity
                                // TODO: Can't input a decimal number on this input
                                <div class="flex-1 min-w-[200px]">
                                    <div class="label p-0">
                                        <span class="label-text">"Quantity"</span>
                                    </div>
                                    <input type="text"
                                        class="input input-bordered w-full"
                                        name="quantity"
                                        required
                                        autocomplete="off"
                                        prop:value={move || new_ingredient_model.get().quantity}
                                        on:input=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                                new_ingredient_model.update(|curr| {
                                                    curr.quantity = value;
                                                });
                                            } else {
                                                new_ingredient_model.update(|curr| {
                                                    curr.quantity = 0.0;
                                                });
                                            }
                                        }
                                    />
                                </div>

                                // Recipe: unit
                                <div class="flex-1 min-w-[200px]">
                                    <div class="label p-0">
                                        <span class="label-text">"Unit"</span>
                                    </div>
                                    <input type="text"
                                        class="input input-bordered w-full"
                                        name="unit"
                                        autocomplete="off"
                                        prop:value=move || new_ingredient_model.get().unit.unwrap_or_default()
                                        on:input=move |ev| {
                                            if !event_target_value(&ev).is_empty() {
                                                new_ingredient_model.update(|curr| {
                                                    curr.unit = Some(event_target_value(&ev));
                                                });
                                            } else {
                                                new_ingredient_model.update(|curr| {
                                                    curr.unit = None;
                                                });
                                            }
                                        }
                                    />
                                </div>

                                <button type="submit" class="btn btn-primary mt-[20px] sm:w-min w-full">"Add"</button>
                            </div>
                        </ActionForm>
                    }
                }/>

            </div>
        </div>
    }.into_any()
}
