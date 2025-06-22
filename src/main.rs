// SPDX-License-Identifier: GPL-3.0-only

pub mod api;
pub mod app;
pub mod components;
pub mod models;
pub mod pages;
pub mod utils;

#[cfg(feature = "ssr")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_files::Files;
    use actix_web::*;
    use cocookies::app::*;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_actix::{LeptosRoutes, generate_route_list};
    use leptos_meta::MetaTags;
    use utils::env_utils::EnvOptions;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;

    // Database
    // Ej: export DATABASE_URL="sqlite:cocookies.db"
    let database_url = std::env::var("DATABASE_URL").expect("database URL must be set");
    if let Some(filename) = &database_url.strip_prefix("sqlite:") {
        if !std::path::Path::new(filename).exists() {
            // Create the file
            std::fs::File::create(filename)?;
            println!("Database '{filename}' created.");
        } else {
            println!("Database '{filename}' already exists.");
        }
    };
    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("cannot create db pool");

    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("Migrations run successfully"),
        Err(err) => {
            eprintln!("Error occurred running migrations: {err}");
            std::process::exit(1);
        }
    };

    let _shared_env_options = SharedValue::new(EnvOptions::get);

    println!("listening on http://{}", &addr);

    HttpServer::new(move || {
        // Generate the list of routes in your Leptos App
        let routes = generate_route_list(App);
        let leptos_options = &conf.leptos_options;
        let site_root = leptos_options.site_root.clone().to_string();

        App::new()
            // database
            .app_data(web::Data::new(pool.clone()))
            // serve JS/WASM/CSS from `pkg`
            .service(Files::new("/pkg", format!("{site_root}/pkg")))
            // serve other assets from the `assets` directory
            .service(Files::new("/assets", &site_root))
            // serve the app uploads
            .service(serve_image_uploads)
            // serve the favicon from /favicon.ico
            .service(favicon)
            .leptos_routes(routes, {
                let leptos_options = leptos_options.clone();
                move || {
                    view! {
                        <!DOCTYPE html>
                        <html lang="en">
                            <head>
                                <meta charset="utf-8"/>
                                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                                <AutoReload options=leptos_options.clone() />
                                <HydrationScripts options=leptos_options.clone()/>
                                <MetaTags/>
                            </head>
                            <body>
                                <App/>
                            </body>
                        </html>
                    }
                }
            })
            .app_data(web::Data::new(leptos_options.to_owned()))
        //.wrap(middleware::Compress::default())
    })
    .bind(&addr)?
    .run()
    .await
}

#[cfg(feature = "ssr")]
#[actix_web::get("favicon.ico")]
async fn favicon(
    leptos_options: actix_web::web::Data<leptos::config::LeptosOptions>,
) -> actix_web::Result<actix_files::NamedFile> {
    let leptos_options = leptos_options.into_inner();
    let site_root = &leptos_options.site_root;
    Ok(actix_files::NamedFile::open(format!(
        "{site_root}/favicon.ico"
    ))?)
}

#[cfg(not(any(feature = "ssr", feature = "csr")))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
    // see optional feature `csr` instead
}

#[cfg(all(not(feature = "ssr"), feature = "csr"))]
pub fn main() {
    // a client-side main function is required for using `trunk serve`
    // prefer using `cargo leptos serve` instead
    // to run: `trunk serve --open --features csr`
    use cocookies::app::*;

    console_error_panic_hook::set_once();

    leptos::mount_to_body(App);
}

#[actix_web::get("/app/uploads/{filename}")]
async fn serve_image_uploads(filename: actix_web::web::Path<String>) -> actix_web::HttpResponse {
    use actix_web::HttpResponse;

    let env_options = utils::env_utils::EnvOptions::get();
    let filepath = format!("{}/{}", env_options.upload_dir, filename.into_inner());

    match std::fs::read(filepath) {
        Ok(data) => HttpResponse::Ok().content_type("image/*").body(data),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}
