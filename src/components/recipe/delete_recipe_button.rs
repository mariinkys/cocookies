// SPDX-License-Identifier: GPL-3.0-only

use crate::components::dialog::DialogComponent;
use crate::components::toast::ToastMessage;
use crate::components::toast::ToastType;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::api::recipe::DeleteRecipe;

#[component]
pub fn DeleteRecipeButton(recipe_id: i32) -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let delete_recipe = ServerAction::<DeleteRecipe>::new();
    let dialog_ref_node: NodeRef<leptos::html::Dialog> = NodeRef::new();

    // 'value' holds the latest *returned* value from the server
    let value = delete_recipe.value();
    Effect::new(move |_| {
        if let Some(val) = value.get() {
            let navigate = use_navigate();
            match val {
                Ok(_) => {
                    dialog_ref_node.get().unwrap().close();
                    set_toast.set(ToastMessage {
                        message: String::from("Deleted Successfully"),
                        toast_type: ToastType::Success,
                        visible: true,
                    });
                    navigate("/", Default::default());
                }
                Err(err) => {
                    set_toast.set(ToastMessage {
                        message: format!("Error {err}"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                }
            }
        }
    });

    view! {
        <div class="fixed bottom-4 right-4 z-10">
            <div class="bg-primary text-white px-4 py-2 rounded-full shadow-lg hover:bg-pink-400 focus:outline-none focus:ring-2 focus:ring-pink-400 flex items-center justify-center">
                <button
                    type="button"
                    class="absolute inset-0 w-full h-full bg-transparent border-none cursor-pointer"
                    on:click=move |_| {
                        let _ = dialog_ref_node.get().unwrap().show_modal();
                    }>
                </button>

                <svg class="w-10 h-10 md:w-20 md:h-20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M18 6L17.1991 18.0129C17.129 19.065 17.0939 19.5911 16.8667 19.99C16.6666 20.3412 16.3648 20.6235 16.0011 20.7998C15.588 21 15.0607 21 14.0062 21H9.99377C8.93927 21 8.41202 21 7.99889 20.7998C7.63517 20.6235 7.33339 20.3412 7.13332 19.99C6.90607 19.5911 6.871 19.065 6.80086 18.0129L6 6M4 6H20M16 6L15.7294 5.18807C15.4671 4.40125 15.3359 4.00784 15.0927 3.71698C14.8779 3.46013 14.6021 3.26132 14.2905 3.13878C13.9376 3 13.523 3 12.6936 3H11.3064C10.477 3 10.0624 3 9.70951 3.13878C9.39792 3.26132 9.12208 3.46013 8.90729 3.71698C8.66405 4.00784 8.53292 4.40125 8.27064 5.18807L8 6M14 10V17M10 10V17" stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                </svg>
            </div>
        </div>

        <DialogComponent dialog_title="Delete Recipe" dialog_node_ref=dialog_ref_node dialog_content=move || {
            view! {
                <ActionForm action=delete_recipe>
                        <div class="w-full flex flex-col gap-2">
                            // We need the id for the update but we don't want to show it.
                            <input type="hidden" name="recipe_id" autocomplete="off" prop:value=recipe_id/>

                            <div>
                                <p class="text-center font-bold text-xl text-error">"Are you sure you want to delete this recipe?"</p>
                                <p class="text-center font-bold text-xl text-error">"This action can't be undone"</p>
                            </div>

                            <button type="submit" class="btn btn-error w-full">"Delete"</button>
                        </div>
                </ActionForm>
            }
        }/>
    }.into_any()
}
