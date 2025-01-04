use leptos::prelude::*;

use crate::{api::recipe::get_full_recipe, components::page_loading::PageLoadingComponent};

#[component]
pub fn ViewFullRecipeComponent(recipe_id: i32) -> impl IntoView {
    let recipe_resource = Resource::new(
        move || recipe_id,
        move |recipe_id| async move { get_full_recipe(recipe_id).await },
    );

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-3xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>

            { move || {
                recipe_resource.get().map(move |x| {
                    x.map(move |recipe_result| {
                        view! {
                            // MAIN RECIPE VIEW
                            <div class="w-full card shadow-xl">
                                <div class="card-body">
                                    <div class="flex gap-3">
                                        <img class="w-48 h-48 object-cover shadow-inner rounded-full" src=format!("../{}", recipe_result.main_photo.unwrap_or_default())/>
                                        <div class="flex flex-col justify-between">
                                            <div>
                                                <h1 class="text-4xl font-bold">{recipe_result.name}</h1>
                                                <p>{recipe_result.description}</p>
                                            </div>
                                            <div class="flex gap-3">
                                                <Show
                                                    when=move || { recipe_result.servings.is_some() }
                                                    fallback=|| view! { "" }
                                                >
                                                    <div class="badge badge-primary font-bold">{recipe_result.servings.unwrap_or_default()}" servings"</div>
                                                </Show>
                                                <Show
                                                    when=move || { recipe_result.prep_time_minutes.is_some() }
                                                    fallback=|| view! { "" }
                                                >
                                                    <div class="badge badge-secondary font-bold">{recipe_result.prep_time_minutes.unwrap_or_default()}"min"</div>
                                                </Show>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>

                            // INGREDIENTS VIEW

                            // STEPS VIEW
                        }
                    })
                })
            }}

            </ErrorBoundary>
        </Suspense>
    }
}
