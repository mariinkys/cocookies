use leptos::prelude::*;

use crate::components::page_loading::PageLoadingComponent;
use crate::utils::EnvOptions;
use crate::{api::recipe::get_all_recipes, models::recipe::Recipe};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let env_options: ReadSignal<EnvOptions> = expect_context();
    let recipes = OnceResource::new(get_all_recipes());

    view! {
        <Suspense fallback=move || view! { <PageLoadingComponent/> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>
                <Show  when=move || {recipes.get().is_some_and(|res| res.is_ok_and(|recipes| !recipes.is_empty()))}
                fallback=move || view! {
                    <div class="flex flex-col items-center justify-center h-[90vh] text-center gap-5">
                        <img src="/assets/utils/NoRecipes.png" />
                        <p class="text-2xl">"No recipes..."</p>
                        <a href="/recipes/new">
                            <button type="button" class="btn btn-primary">"Add a new one!"</button>
                        </a>
                    </div>

                }>
                    // TODO: Maybe new recipe could also be a dialog and not a single almost empty page
                    <div class="fixed bottom-4 right-4 z-10">
                        <a href="/recipes/new" class="bg-primary text-white px-4 py-2 rounded-full shadow-lg hover:bg-pink-400 focus:outline-none focus:ring-2 focus:ring-pink-400 flex items-center justify-center">
                        <svg class="w-20 h-20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                            <path d="M4 12H20M12 4V20" stroke="#ffffff" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
                        </a>
                    </div>

                    <div class="p-8 flex gap-5 flex-wrap justify-start sm:justify-center">
                        <For
                            each=move || {recipes.get_untracked().and_then(|res| res.ok()).unwrap_or_default()}
                            key=|recipe| recipe.recipe_id
                            children=move |recipe: Recipe| {
                                view! {
                                    <div class="card bg-base-100 w-96 shadow-xl h-max">
                                        <a class="w-full h-full" href=format!("recipes/{}", recipe.recipe_id.unwrap_or_default())>
                                            <figure>
                                                <img
                                                src=format!("{}/{}", env_options.get().upload_dir, recipe.main_photo.unwrap_or_default())/>
                                            </figure>
                                            <div class="card-body">
                                                <h2 class="card-title">{recipe.name}</h2>
                                                <p>{match recipe.description {
                                                    Some(desc) => desc,
                                                    None => String::from("No description"),
                                                }}
                                                </p>
                                                <div class="flex gap-3 flex-wrap justify-end">
                                                    <Show
                                                        when=move || { recipe.servings.is_some() }
                                                        fallback=|| view! { "" }
                                                    >
                                                        <div class="badge badge-primary font-bold">{recipe.servings.unwrap_or_default()}" servings"</div>
                                                    </Show>
                                                    <Show
                                                        when=move || { recipe.prep_time_minutes.is_some() }
                                                        fallback=|| view! { "" }
                                                    >
                                                        <div class="badge badge-secondary font-bold">{recipe.prep_time_minutes.unwrap_or_default()}"min"</div>
                                                    </Show>
                                                </div>
                                            </div>
                                        </a>
                                    </div>
                                }
                            }
                        />
                    </div>
                </Show>
            </ErrorBoundary>
        </Suspense>
    }
}
