use leptos::prelude::*;

use crate::api::recipe_ingredients::get_all_recipe_ingredients;

#[component]
pub fn ViewEditIngredientsComponent(recipe_id: i32) -> impl IntoView {
    let recipe_ingredients_resource = Resource::new(
        move || recipe_id,
        move |recipe_id| async move { get_all_recipe_ingredients(recipe_id).await },
    );

    view! {
        <p>"ViewEditIngredientsComponent"</p>
        { move || {
            recipe_ingredients_resource.get().map(move |x| {
                x.map(move |recipe_ingredient_result| {
                    view! {
                        <p>"VAIA"</p>
                    }
                })
            })
        }}
    }
}
