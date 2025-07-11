// SPDX-License-Identifier: GPL-3.0-only

use leptos::prelude::*;
use leptos_meta::{Link, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment, WildcardSegment,
    components::{Route, Router, Routes},
    path,
};

use crate::components::toast::ToastComponent;
use crate::pages::homepage::HomePage;
use crate::pages::new_recipe::NewRecipe;
use crate::pages::settings::SettingsPage;
use crate::pages::view_edit_recipe::ViewEditRecipe;
use crate::{components::navbar::NavbarComponent, utils::env_utils::EnvOptions};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Provides context for things that change between docker and local envs
    let (read, _write) = signal::<EnvOptions>(SharedValue::new(EnvOptions::get).into_inner());
    provide_context::<ReadSignal<EnvOptions>>(read);

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/cocookies.css"/>

        // sets the document title
        <Title text="Cocookies"/>

        <Link rel="apple-touch-icon" sizes="180x180" href="/assets/apple-touch-icon.png" />
        <Link rel="icon" type_="image/png" sizes="32x32" href="/assets/favicon-32x32.png" />
        <Link rel="icon" type_="image/png" sizes="16x16" href="/assets/favicon-16x16.png" />
        <Link rel="manifest" href="/assets/site.webmanifest" />

        <ToastComponent/>
        <NavbarComponent/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=path!("/recipes/new") view=NewRecipe/>
                    <Route path=path!("/recipes/:id") view=ViewEditRecipe/>
                    <Route path=path!("/settings") view=SettingsPage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
