use leptos::prelude::*;

use crate::{
    api::recipe_steps::{
        get_all_recipe_steps, AddRecipeSteps, DeleteRecipeStep, UpdateRecipeSteps,
    },
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    models::recipe_step::RecipeStep,
};

#[component]
pub fn ViewEditStepsComponent(recipe_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let new_step_model = RwSignal::new(RecipeStep::init(recipe_id));
    let recipe_steps_resource = Resource::new(
        move || recipe_id,
        move |recipe_id| async move { get_all_recipe_steps(recipe_id).await },
    );

    let update_recipe_step = ServerAction::<UpdateRecipeSteps>::new();
    let update_value = update_recipe_step.value();
    Effect::new(move |_| {
        if let Some(val) = update_value() {
            match val {
                Ok(_) => {
                    // TODO: Close modal, maybe refetch to update? Why it doesn't update if it's a signal...
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
    let add_recipe_step = ServerAction::<AddRecipeSteps>::new();
    let add_value = add_recipe_step.value();
    Effect::new(move |_| {
        if let Some(val) = add_value() {
            match val {
                Ok(_) => {
                    new_step_model.set(RecipeStep::init(recipe_id));
                    recipe_steps_resource.refetch();
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

    let delete_recipe_step = ServerAction::<DeleteRecipeStep>::new();
    let delete_value = delete_recipe_step.value();
    Effect::new(move |_| {
        if let Some(val) = delete_value() {
            match val {
                Ok(_) => {
                    recipe_steps_resource.refetch();
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
                    <h1 class="text-4xl font-bold">"Steps"</h1>
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
                            recipe_steps_resource.get().map(move |x| {
                                x.map(move |recipe_step_result| {
                                    let recipe_steps = RwSignal::new(recipe_step_result);
                                    view! {
                                        <For each=move || recipe_steps.get() key=|step| step.step_id children=move |step| {
                                            let model = RwSignal::new(step);
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
                                                    <p>{model.read_only().get().instructions}</p>
                                                </div>

                                                <DialogComponent dialog_title="Edit Step" dialog_node_ref=update_dialog_ref dialog_content=move || {
                                                    view! {
                                                        <ActionForm action=update_recipe_step>
                                                            <div class="flex flex-wrap gap-2 items-center">
                                                                // We need the id for the update but we don't want to show it.
                                                                <input type="hidden" name="step_id" autocomplete="off" prop:value={move || model.get().step_id.unwrap_or_default()}/>

                                                                // Recipe Step: step_number
                                                                <div class="flex-1 min-w-[200px]">
                                                                    <div class="label p-0">
                                                                        <span class="label-text">"Step Number"</span>
                                                                    </div>
                                                                    <input type="number"
                                                                        class="input input-bordered w-full"
                                                                        name="step_number"
                                                                        required
                                                                        autocomplete="off"
                                                                        prop:value={move || model.get().step_number}
                                                                        on:input=move |ev| {
                                                                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                                                                model.update(|curr| {
                                                                                    curr.step_number = value;
                                                                                });
                                                                            } else {
                                                                                model.update(|curr| {
                                                                                    curr.step_number = 0;
                                                                                });
                                                                            }
                                                                        }
                                                                    />
                                                                </div>

                                                                // TODO: This should be a textarea
                                                                // Recipe Step: instructions
                                                                <div class="flex-1 min-w-[200px]">
                                                                    <div class="label p-0">
                                                                        <span class="label-text">"Instructions"</span>
                                                                    </div>
                                                                    <input type="text"
                                                                        class="input input-bordered w-full"
                                                                        name="instructions"
                                                                        required
                                                                        autocomplete="off"
                                                                        prop:value={move || model.get().instructions}
                                                                        on:input=move |ev| {
                                                                            model.update(|curr| {
                                                                                curr.instructions = event_target_value(&ev);
                                                                            });
                                                                        }
                                                                    />
                                                                </div>

                                                                // TODO: Fix width view
                                                                <div class="sm:w-min w-full">
                                                                    <ActionForm action=delete_recipe_step>
                                                                        // We need the id for the update but we don't want to show it.
                                                                        <input type="hidden" name="step_id" autocomplete="off" prop:value={move || model.get().step_id.unwrap_or_default()}/>

                                                                        <button type="submit" class="btn btn-warning mt-[20px] sm:w-min w-full">"Delete"</button>
                                                                    </ActionForm>
                                                                </div>

                                                                <button type="submit" class="btn btn-primary mt-[20px] sm:w-min w-full">"Update"</button>
                                                            </div>
                                                        </ActionForm>
                                                    }
                                                }/>
                                            }
                                        }/>


                                    }
                                })
                            })
                        }}
                    </ErrorBoundary>
                </Suspense>

                <DialogComponent dialog_title="Add Step" dialog_node_ref=add_dialog_ref_node dialog_content=move || {
                    view! {
                        <ActionForm action=add_recipe_step>
                            <div class="flex flex-wrap gap-2 items-center">
                                // We need the id for the update but we don't want to show it.
                                <input type="hidden" name="recipe_id" autocomplete="off" prop:value={move || new_step_model.get().recipe_id}/>

                                // Recipe Step: step_number
                                <div class="flex-1 min-w-[200px]">
                                    <div class="label p-0">
                                        <span class="label-text">"Step Number"</span>
                                    </div>
                                    <input type="number"
                                        class="input input-bordered w-full"
                                        name="step_number"
                                        required
                                        autocomplete="off"
                                        prop:value={move || new_step_model.get().step_number}
                                        on:input=move |ev| {
                                            if let Ok(value) = event_target_value(&ev).parse::<i32>() {
                                                new_step_model.update(|curr| {
                                                    curr.step_number = value;
                                                });
                                            } else {
                                                new_step_model.update(|curr| {
                                                    curr.step_number = 0;
                                                });
                                            }
                                        }
                                    />
                                </div>

                                // Recipe Step: instructions
                                <div class="flex-1 min-w-[200px]">
                                    <div class="label p-0">
                                        <span class="label-text">"Instructions"</span>
                                    </div>
                                    <input type="text"
                                        class="input input-bordered w-full"
                                        name="instructions"
                                        required
                                        autocomplete="off"
                                        prop:value={move || new_step_model.get().instructions}
                                        on:input=move |ev| {
                                            new_step_model.update(|curr| {
                                                curr.instructions = event_target_value(&ev);
                                            });
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
    }
}
