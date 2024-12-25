use crate::components::page_loading::PageLoadingComponent;
use crate::components::recipe::add_edit_ingredients::ViewEditIngredientsComponent;
use crate::components::recipe::add_edit_steps::ViewEditStepsComponent;
use crate::components::recipe::edit_recipe::ViewEditRecipeComponent;
use crate::components::toast::{ToastMessage, ToastType};
use leptos::prelude::*;
use leptos_router::hooks::use_params;
use leptos_router::params::Params;

#[derive(Params, PartialEq)]
struct RecipeParams {
    id: Option<usize>,
}

#[component]
pub fn ViewEditRecipe() -> impl IntoView {
    let set_toast: WriteSignal<ToastMessage> = expect_context();
    let params = use_params::<RecipeParams>();

    let id = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|params| params.id)
            .unwrap_or_default()
    };
    if id() == 0 {
        set_toast.set(ToastMessage {
            message: String::from("Missing or Invalid Id Params"),
            toast_type: ToastType::Error,
            visible: true,
        });
        // TODO: Can't redirect with navigate cuz crashes (thread 'actix-rt|system:0|arbiter:8' panicked)
        // It works inside a spawn_local but still gives the error so better not to?
        // spawn_local(async {
        //  navigate("/", Default::default());
        //});
    } else {
        // Pass the recipe id to all children components
        provide_context(id());
    }

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>
                <div class="flex">
                    // EDIT RECIPE COMPONENT (TOP LEFT)
                    <ViewEditRecipeComponent/>

                    // EDIT/ADD INGREDIENTS COMPONENT (TOP RIGHT)
                    <ViewEditIngredientsComponent/>
                </div>
                // EDIT/ADD STEPS COMPONENT (BOTTOM)
                <ViewEditStepsComponent/>
            </ErrorBoundary>
        </Suspense>
    }
}
