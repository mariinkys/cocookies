use leptos::prelude::*;

use crate::{api::recipe::get_all_recipes, models::recipe::Recipe};

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    let recipes = OnceResource::new(get_all_recipes());

    view! {
        <Suspense fallback=move || view! { <p>"Loading..."</p> }>
            <ErrorBoundary fallback=|error| view! {
                <p class="text-xl text-center text-red-500">"An error occurred: " {format!("{:?}", error)}</p>
            }>
                <Show  when=move || {recipes.get().is_some_and(|res| res.is_ok_and(|recipes| !recipes.is_empty()))}
                fallback=move || view! { <p class="text-xl text-center">"No recipes..."</p>}>
                    <div class="p-8 flex gap-5 flex-wrap justify-start sm:justify-center items-center">
                        <For
                            each=move || {recipes.get_untracked().and_then(|res| res.ok()).unwrap_or_default()}
                            key=|recipe| recipe.recipe_id
                            children=move |recipe: Recipe| {
                                view! {
                                    <div class="card bg-base-100 w-96 shadow-xl">
                                        <a class="w-full h-full" href=format!("recipes/{}", recipe.recipe_id.unwrap_or_default())>
                                            <figure>
                                                <img
                                                src="https://img.daisyui.com/images/stock/photo-1606107557195-0e29a4b5b4aa.webp"/>
                                            </figure>
                                            <div class="card-body">
                                                <h2 class="card-title">{recipe.name}</h2>
                                                <p>{match recipe.description {
                                                    Some(desc) => desc,
                                                    None => String::from("No description"),
                                                }}
                                                </p>
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
