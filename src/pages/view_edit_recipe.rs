use crate::api::recipe::get_recipe;
use crate::components::page_loading::PageLoadingComponent;
use crate::components::recipe::add_edit_ingredients::ViewEditIngredientsComponent;
use crate::components::recipe::add_edit_steps::ViewEditStepsComponent;
use crate::components::recipe::delete_recipe_button::DeleteRecipeButton;
use crate::components::recipe::edit_recipe::ViewEditRecipeComponent;
use crate::components::toast::{ToastMessage, ToastType};
use crate::models::recipe::Recipe;
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct RecipeParams {
    id: Option<i32>,
}

#[component]
pub fn ViewEditRecipe() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let params = use_params::<RecipeParams>();

    let recipe_resource = Resource::new(
        move || params.read().as_ref().ok().and_then(|params| params.id),
        move |params| async move {
            match params {
                Some(id) => get_recipe(id).await,
                None => {
                    // TODO: The Toast does not show if navigating through the search bar, it works when navigating through an anhor tag
                    // Also we should naviagte back to HomePage
                    leptos::logging::log!("Bad Params");
                    set_toast.set(ToastMessage {
                        message: String::from("Bad Params"),
                        toast_type: ToastType::Error,
                        visible: true,
                    });
                    Ok(Recipe::default())
                }
            }
        },
    );

    view! {
        <div class="px-8">
            <Suspense fallback=move || view! { <PageLoadingComponent/> }>
                <ErrorBoundary fallback=|error| view! {
                    <p class="text-3xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
                }>
                    // TODO Maybe we can use either and check the recipe id? Also maybe keep an edit/view state in this page and switch the view?
                    { move || {
                        recipe_resource.get().map(move |x| {
                            x.map(move |recipe_result| {
                                let recipe_id = recipe_result.recipe_id.unwrap_or_default();

                                view! {
                                    // EDIT RECIPE
                                    <ViewEditRecipeComponent recipe=recipe_result/>

                                    // EDIT/ADD INGREDIENTS COMPONENT
                                    <ViewEditIngredientsComponent recipe_id=recipe_id/>
                                    // EDIT/ADD STEPS COMPONENT
                                    <ViewEditStepsComponent/>

                                    // DELETE RECIPE BUTTON
                                    <DeleteRecipeButton recipe_id=recipe_id/>
                                }
                            })
                        })
                    }}
                </ErrorBoundary>
            </Suspense>
        </div>
    }
}
