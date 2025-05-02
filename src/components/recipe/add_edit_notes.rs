use leptos::prelude::*;

use crate::{
    api::recipe_notes::{
        AddRecipeNotes, DeleteRecipeNote, UpdateRecipeNotes, get_all_recipe_notes,
    },
    components::{
        dialog::DialogComponent,
        page_loading::PageLoadingComponent,
        toast::{ToastMessage, ToastType},
    },
    models::recipe_note::RecipeNote,
};

#[component]
pub fn ViewEditNotesComponent(recipe_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();

    let new_note_model = RwSignal::new(RecipeNote::init(recipe_id));
    let recipe_notes_resource = Resource::new(
        move || recipe_id,
        move |recipe_id| async move { get_all_recipe_notes(recipe_id).await },
    );

    let update_recipe_note = ServerAction::<UpdateRecipeNotes>::new();
    let update_value = update_recipe_note.value();
    Effect::new(move |_| {
        if let Some(val) = update_value.get() {
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
    let add_recipe_note = ServerAction::<AddRecipeNotes>::new();
    let add_value = add_recipe_note.value();
    Effect::new(move |_| {
        if let Some(val) = add_value.get() {
            match val {
                Ok(_) => {
                    new_note_model.set(RecipeNote::init(recipe_id));
                    recipe_notes_resource.refetch();
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

    let delete_recipe_note = ServerAction::<DeleteRecipeNote>::new();
    let delete_value = delete_recipe_note.value();
    Effect::new(move |_| {
        if let Some(val) = delete_value.get() {
            match val {
                Ok(_) => {
                    recipe_notes_resource.refetch();
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
                    <h1 class="text-4xl font-bold">"Notes"</h1>
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
                            recipe_notes_resource.get().map(move |x| {
                                x.map(move |recipe_note_result| {
                                    let recipe_notes = RwSignal::new(recipe_note_result);
                                    view! {
                                        <For each=move || recipe_notes.get() key=|note| note.note_id children=move |note| {
                                            let model = RwSignal::new(note);
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
                                                    <p>{move || model.read_only().get().note}</p>
                                                </div>

                                                <DialogComponent dialog_title="Edit Note" dialog_node_ref=update_dialog_ref dialog_content=move || {
                                                    view! {
                                                        <ActionForm action=update_recipe_note>
                                                            <div class="flex flex-col gap-2 w-full">
                                                                // We need the id for the update but we don't want to show it.
                                                                <input type="hidden" name="note_id" autocomplete="off" prop:value={move || model.get().note_id.unwrap_or_default()}/>

                                                                // Recipe Note: note
                                                                <div class="w-full">
                                                                    <fieldset class="fieldset">
                                                                        <label class="label" for="note">"Note"</label>
                                                                        <textarea
                                                                            class="textarea textarea-bordered w-full min-h-80"
                                                                            name="note"
                                                                            id="note"
                                                                            required
                                                                            prop:value={move || model.get().note}
                                                                            on:input=move |ev| {
                                                                                model.update(|curr| {
                                                                                    curr.note = event_target_value(&ev);
                                                                                });
                                                                            }
                                                                        />
                                                                    </fieldset>
                                                                </div>

                                                                <div class="w-full">
                                                                    <ActionForm action=delete_recipe_note>
                                                                        // We need the id for the update but we don't want to show it.
                                                                        <input type="hidden" name="note_id" autocomplete="off" prop:value={move || model.get().note_id.unwrap_or_default()}/>

                                                                        <button type="submit" class="btn btn-warning w-full">"Delete"</button>
                                                                    </ActionForm>
                                                                </div>

                                                                <button type="submit" class="btn btn-primary w-full">"Update"</button>
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

                <DialogComponent dialog_title="Add Note" dialog_node_ref=add_dialog_ref_node dialog_content=move || {
                    view! {
                        <ActionForm action=add_recipe_note>
                            <div class="flex flex-col gap-2 w-full">
                                // We need the id for the update but we don't want to show it.
                                <input type="hidden" name="recipe_id" autocomplete="off" prop:value={move || new_note_model.get().recipe_id}/>

                                // Recipe Note: note
                                <div class="w-full">
                                    <fieldset class="fieldset">
                                        <label class="label" for="note">"Note"</label>
                                        <textarea
                                            class="textarea textarea-bordered w-full min-h-80"
                                            name="note"
                                            id="note"
                                            required
                                            prop:value={move || new_note_model.get().note}
                                            on:input=move |ev| {
                                                new_note_model.update(|curr| {
                                                    curr.note = event_target_value(&ev);
                                                });
                                            }
                                        />
                                    </fieldset>
                                </div>

                                <button type="submit" class="btn btn-primary w-full">"Add"</button>
                            </div>
                        </ActionForm>
                    }
                }/>
            </div>
        </div>
    }.into_any()
}
