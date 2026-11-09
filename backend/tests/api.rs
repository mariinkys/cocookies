//! End-to-end tests: the real router, a real SQLite file, no network.

use axum::{
    Router,
    body::{Body, to_bytes},
    http::{HeaderMap, Method, Request, StatusCode, header},
};
use cocookies::infra::{app::create_app, config::AppConfig, setup::init_app_state};
use serde_json::{Value, json};
use tower::ServiceExt;

struct TestApp {
    router: Router,
    db_path: std::path::PathBuf,
}

impl Drop for TestApp {
    fn drop(&mut self) {
        for suffix in ["", "-wal", "-shm"] {
            let _ = std::fs::remove_file(format!("{}{suffix}", self.db_path.display()));
        }
    }
}

struct Res {
    status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
}

impl Res {
    fn json(&self) -> Value {
        serde_json::from_slice(&self.body).unwrap_or(Value::Null)
    }

    /// `access_token=...; refresh_token=...` built from the Set-Cookie headers.
    fn cookies(&self) -> String {
        self.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok()?.split(';').next().map(str::to_owned))
            .filter(|kv| !kv.ends_with('='))
            .collect::<Vec<_>>()
            .join("; ")
    }
}

async fn app_with(configure: impl FnOnce(&mut AppConfig)) -> TestApp {
    let db_path = std::env::temp_dir().join(format!("cocookies-test-{}.db", uuid::Uuid::new_v4()));
    let mut config = AppConfig::for_tests(db_path.clone());
    configure(&mut config);
    let state = init_app_state(config).await.expect("state");
    TestApp {
        router: create_app(state),
        db_path,
    }
}

async fn app() -> TestApp {
    app_with(|_| {}).await
}

impl TestApp {
    async fn call(
        &self,
        method: Method,
        uri: &str,
        body: Option<Value>,
        cookie: Option<&str>,
    ) -> Res {
        let mut req = Request::builder().method(method).uri(uri);
        if let Some(c) = cookie {
            req = req.header(header::COOKIE, c);
        }
        let req = match body {
            Some(b) => req
                .header(header::CONTENT_TYPE, "application/json")
                .body(Body::from(b.to_string())),
            None => req.body(Body::empty()),
        }
        .unwrap();
        let res = self.router.clone().oneshot(req).await.unwrap();
        let status = res.status();
        let headers = res.headers().clone();
        let body = to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap()
            .to_vec();
        Res {
            status,
            headers,
            body,
        }
    }

    /// Registers and logs in; returns (user id, cookie header).
    async fn user(&self, name: &str) -> (String, String) {
        let email = format!("{name}@example.com");
        let r = self
            .call(
                Method::POST,
                "/api/auth/register",
                Some(json!({"name": name, "email": email, "password": "password123"})),
                None,
            )
            .await;
        assert_eq!(
            r.status,
            StatusCode::CREATED,
            "{}",
            String::from_utf8_lossy(&r.body)
        );
        let r = self
            .call(
                Method::POST,
                "/api/auth/login",
                Some(json!({"email": email, "password": "password123"})),
                None,
            )
            .await;
        assert_eq!(r.status, StatusCode::OK);
        (r.json()["id"].as_str().unwrap().to_owned(), r.cookies())
    }
}

fn recipe_body(title: &str, shared: bool) -> Value {
    json!({
        "title": title,
        "description": "Crispy and golden. A classic for any occasion, and it reheats beautifully the next day.",
        "categoryId": null,
        "difficultyId": "",
        "prepTime": 15,
        "cookTime": 30,
        "servings": 4,
        "imageUrl": null,
        "shared": shared,
        "steps": [
            {"stepNumber": 2, "instructions": "Fry the potatoes slowly in plenty of olive oil until tender but not browned. Drain, keeping the oil.", "duration": 20},
            {"stepNumber": 1, "instructions": "Peel and thinly slice the potatoes and the onion.", "duration": null},
            {"stepNumber": 3, "instructions": "Beat the eggs with salt, fold in the potatoes and onion and let them rest for a few minutes.\nCook on both sides.", "duration": 10}
        ],
        "ingredients": [
            {"name": "Potatoes", "quantity": 1, "unit": "kg", "notes": "waxy", "sortOrder": 0},
            {"name": "Eggs", "quantity": 6, "unit": null, "notes": null, "sortOrder": 1},
            {"name": "Onion", "quantity": 0.5, "unit": null, "notes": "optional, but recommended", "sortOrder": 2},
            {"name": "Olive oil", "quantity": null, "unit": null, "notes": null, "sortOrder": 3}
        ],
        "nutrition": {"servingSizeValue": 100, "servingSizeUnit": "g", "calories": 180.5, "proteinG": 6, "carbsG": 15,
                      "sugarG": 1, "fatG": 11, "saturatedFatG": 2, "fiberG": 1.5, "sodiumMg": 300}
    })
}

#[tokio::test]
async fn system_endpoints_are_public() {
    let app = app().await;
    let r = app
        .call(Method::GET, "/api/system/version", None, None)
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(String::from_utf8_lossy(&r.body), env!("CARGO_PKG_VERSION"));

    let r = app
        .call(Method::GET, "/api/system/config", None, None)
        .await;
    assert_eq!(r.json()["guestAccess"], true);

    let r = app
        .call(Method::GET, "/api/system/health", None, None)
        .await;
    assert_eq!(r.status, StatusCode::OK);

    let r = app.call(Method::GET, "/api/nope", None, None).await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    assert_eq!(r.json()["status"], 404);
}

#[tokio::test]
async fn auth_flow_and_errors() {
    let app = app().await;

    // Validation errors use the standard JSON shape.
    let r = app
        .call(
            Method::POST,
            "/api/auth/register",
            Some(json!({"name": "x", "email": "bad", "password": "short"})),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let msg = r.json()["message"].as_str().unwrap().to_owned();
    assert!(
        msg.contains("Must be a valid email") && msg.contains("at least 8"),
        "{msg}"
    );

    let (id, cookies) = app.user("ana").await;
    assert!(cookies.contains("access_token=") && cookies.contains("refresh_token="));

    // Duplicate e-mail → 409.
    let r = app
        .call(
            Method::POST,
            "/api/auth/register",
            Some(json!({"name": "Ana", "email": "ana@example.com", "password": "password123"})),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::CONFLICT);

    // Session.
    let r = app
        .call(Method::GET, "/api/users/me", None, Some(&cookies))
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.json()["id"], id.as_str());
    assert!(r.json().get("password").is_none());
    let r = app.call(Method::GET, "/api/users/me", None, None).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    assert_eq!(r.json()["message"], "Authentication required");

    // Wrong password.
    let r = app
        .call(
            Method::POST,
            "/api/auth/login",
            Some(json!({"email": "ana@example.com", "password": "nope-nope"})),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    assert_eq!(r.json()["message"], "Invalid email or password");

    // Refresh rotates; reusing the old refresh token revokes everything.
    let r = app
        .call(Method::POST, "/api/auth/refresh", None, Some(&cookies))
        .await;
    assert_eq!(r.status, StatusCode::OK);
    let rotated = r.cookies();
    assert_ne!(rotated, cookies);
    let r = app
        .call(Method::POST, "/api/auth/refresh", None, Some(&cookies))
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    assert!(r.json()["message"].as_str().unwrap().contains("reuse"));
    let r = app
        .call(Method::POST, "/api/auth/refresh", None, Some(&rotated))
        .await;
    assert_eq!(
        r.status,
        StatusCode::UNAUTHORIZED,
        "rotated token must be revoked after reuse"
    );

    // Logout clears cookies and works even without a session.
    let r = app
        .call(Method::POST, "/api/auth/logout", None, Some(&rotated))
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    assert!(
        r.headers
            .get_all(header::SET_COOKIE)
            .iter()
            .any(|v| v.to_str().unwrap().contains("Max-Age=0"))
    );
    let r = app.call(Method::POST, "/api/auth/logout", None, None).await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn account_lockout() {
    let app = app().await;
    app.user("bob").await;
    for _ in 0..5 {
        let r = app
            .call(
                Method::POST,
                "/api/auth/login",
                Some(json!({"email": "bob@example.com", "password": "wrong-pass"})),
                None,
            )
            .await;
        assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    }
    // Even the right password is refused while locked.
    let r = app
        .call(
            Method::POST,
            "/api/auth/login",
            Some(json!({"email": "bob@example.com", "password": "password123"})),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    assert!(r.json()["message"].as_str().unwrap().contains("15 minutes"));
}

#[tokio::test]
async fn users_can_only_touch_themselves() {
    let app = app().await;
    let (ana, ana_c) = app.user("ana").await;
    let (bob, _) = app.user("bob").await;

    let r = app
        .call(
            Method::GET,
            &format!("/api/users/{bob}"),
            None,
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);

    let r = app
        .call(
            Method::PUT,
            &format!("/api/users/{ana}"),
            Some(json!({"name": "Ana M", "email": "bob@example.com"})),
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::CONFLICT);
    let r = app
        .call(
            Method::PUT,
            &format!("/api/users/{ana}"),
            Some(json!({"name": "Ana M", "email": "ana.m@example.com"})),
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.json()["email"], "ana.m@example.com");
    // The session survives an e-mail change (tokens are bound to the id).
    let r = app
        .call(Method::GET, "/api/users/me", None, Some(&ana_c))
        .await;
    assert_eq!(r.json()["email"], "ana.m@example.com");

    let r = app
        .call(
            Method::PATCH,
            &format!("/api/users/{ana}/password"),
            Some(json!({"currentPassword": "bad-bad-bad", "newPassword": "newpassword1"})),
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    let r = app
        .call(
            Method::PATCH,
            &format!("/api/users/{ana}/password"),
            Some(json!({"currentPassword": "password123", "newPassword": "newpassword1"})),
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    let r = app
        .call(
            Method::POST,
            "/api/auth/login",
            Some(json!({"email": "ana.m@example.com", "password": "newpassword1"})),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);

    let r = app
        .call(Method::GET, "/api/users/not-a-uuid", None, Some(&ana_c))
        .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);

    // Deleting the account invalidates its token immediately.
    let r = app
        .call(
            Method::DELETE,
            &format!("/api/users/{ana}"),
            None,
            Some(&ana_c),
        )
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    let r = app
        .call(Method::GET, "/api/users/me", None, Some(&ana_c))
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn categories_and_difficulties() {
    let app = app().await;
    let (_, ana) = app.user("ana").await;
    let (_, bob) = app.user("bob").await;

    for base in ["/api/categories", "/api/difficulties"] {
        let r = app
            .call(
                Method::POST,
                base,
                Some(json!({"name": "Desserts", "description": ""})),
                Some(&ana),
            )
            .await;
        assert_eq!(r.status, StatusCode::CREATED);
        let id = r.json()["id"].as_str().unwrap().to_owned();
        assert_eq!(r.json()["description"], Value::Null);

        let r = app
            .call(
                Method::POST,
                base,
                Some(json!({"name": "Desserts"})),
                Some(&ana),
            )
            .await;
        assert_eq!(r.status, StatusCode::CONFLICT);
        // Names are unique per user only.
        let r = app
            .call(
                Method::POST,
                base,
                Some(json!({"name": "Desserts"})),
                Some(&bob),
            )
            .await;
        assert_eq!(r.status, StatusCode::CREATED);

        app.call(
            Method::POST,
            base,
            Some(json!({"name": "appetizers"})),
            Some(&ana),
        )
        .await;
        let r = app.call(Method::GET, base, None, Some(&ana)).await;
        assert_eq!(r.json()["totalElements"], 2);
        assert_eq!(
            r.json()["content"][0]["name"],
            "appetizers",
            "case-insensitive sort"
        );
        let r = app
            .call(
                Method::GET,
                &format!("{base}?search=dess&sortBy=createdAt&sortDir=desc"),
                None,
                Some(&ana),
            )
            .await;
        assert_eq!(r.json()["totalElements"], 1);
        let r = app
            .call(
                Method::GET,
                &format!("{base}?sortBy=password"),
                None,
                Some(&ana),
            )
            .await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST);

        let r = app
            .call(Method::GET, &format!("{base}/selector"), None, Some(&ana))
            .await;
        assert_eq!(r.json().as_array().unwrap().len(), 2);

        let r = app
            .call(Method::GET, &format!("{base}/{id}"), None, Some(&ana))
            .await;
        assert_eq!(r.status, StatusCode::OK);
        let r = app
            .call(Method::GET, &format!("{base}/{id}"), None, Some(&bob))
            .await;
        assert_eq!(r.status, StatusCode::FORBIDDEN);

        let r = app
            .call(
                Method::PUT,
                &format!("{base}/{id}"),
                Some(json!({"name": "Sweets", "description": "Yum"})),
                Some(&ana),
            )
            .await;
        assert_eq!(r.json()["name"], "Sweets");
        let r = app
            .call(Method::DELETE, &format!("{base}/{id}"), None, Some(&bob))
            .await;
        assert_eq!(r.status, StatusCode::FORBIDDEN);
        let r = app
            .call(Method::DELETE, &format!("{base}/{id}"), None, Some(&ana))
            .await;
        assert_eq!(r.status, StatusCode::NO_CONTENT);
    }
}

#[tokio::test]
async fn recipes_crud_sharing_and_guests() {
    let app = app().await;
    let (ana_id, ana) = app.user("ana").await;
    let (_, bob) = app.user("bob").await;

    let cat = app
        .call(
            Method::POST,
            "/api/categories",
            Some(json!({"name": "Spanish"})),
            Some(&ana),
        )
        .await
        .json();
    let bob_cat = app
        .call(
            Method::POST,
            "/api/categories",
            Some(json!({"name": "Bob's"})),
            Some(&bob),
        )
        .await
        .json();

    // Validation.
    let mut bad = recipe_body("", true);
    bad["steps"] = json!([]);
    bad["servings"] = json!(0);
    let r = app
        .call(Method::POST, "/api/recipes", Some(bad), Some(&ana))
        .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);
    let msg = r.json()["message"].as_str().unwrap().to_owned();
    assert!(
        msg.contains("Title is required")
            && msg.contains("At least one step")
            && msg.contains("Servings"),
        "{msg}"
    );

    // Cannot reference someone else's category.
    let mut body = recipe_body("Tortilla", true);
    body["categoryId"] = bob_cat["id"].clone();
    let r = app
        .call(Method::POST, "/api/recipes", Some(body), Some(&ana))
        .await;
    assert_eq!(r.status, StatusCode::BAD_REQUEST);

    // Create a shared and a private recipe.
    let mut body = recipe_body("Tortilla de patatas", true);
    body["categoryId"] = cat["id"].clone();
    let r = app
        .call(Method::POST, "/api/recipes", Some(body), Some(&ana))
        .await;
    assert_eq!(
        r.status,
        StatusCode::CREATED,
        "{}",
        String::from_utf8_lossy(&r.body)
    );
    let shared = r.json();
    let shared_id = shared["id"].as_str().unwrap().to_owned();
    assert_eq!(shared["userId"], ana_id.as_str());
    assert_eq!(shared["category"]["name"], "Spanish");
    assert_eq!(shared["difficulty"], Value::Null);
    assert_eq!(shared["steps"][0]["stepNumber"], 1, "steps are ordered");
    assert_eq!(shared["ingredients"].as_array().unwrap().len(), 4);
    assert_eq!(shared["nutrition"]["calories"], 180.5);
    assert!(shared["createdAt"].as_str().unwrap().ends_with('Z'));

    let r = app
        .call(
            Method::POST,
            "/api/recipes",
            Some(recipe_body("Secret sauce", false)),
            Some(&ana),
        )
        .await;
    let private_id = r.json()["id"].as_str().unwrap().to_owned();

    // Lists return lightweight cards.
    let r = app
        .call(
            Method::GET,
            "/api/recipes?size=1&sortBy=title",
            None,
            Some(&ana),
        )
        .await;
    let page = r.json();
    assert_eq!(
        (
            page["totalElements"].as_i64(),
            page["totalPages"].as_i64(),
            page["last"].as_bool()
        ),
        (Some(2), Some(2), Some(false))
    );
    assert_eq!(page["content"][0]["title"], "Secret sauce");
    assert!(page["content"][0].get("steps").is_none());
    let r = app
        .call(
            Method::GET,
            "/api/recipes?search=tortilla",
            None,
            Some(&ana),
        )
        .await;
    assert_eq!(r.json()["content"][0]["category"]["name"], "Spanish");
    let r = app
        .call(Method::GET, "/api/recipes", None, Some(&bob))
        .await;
    assert_eq!(r.json()["totalElements"], 0);

    // Guests: shared list and shared recipe (incl. PDF) are readable, everything else is not.
    let r = app
        .call(Method::GET, "/api/recipes/shared", None, None)
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.json()["totalElements"], 1);
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{shared_id}"),
            None,
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{private_id}"),
            None,
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    let r = app.call(Method::GET, "/api/recipes", None, None).await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    let r = app
        .call(
            Method::POST,
            "/api/recipes",
            Some(recipe_body("x", true)),
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::UNAUTHORIZED);
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{shared_id}/pdf/default-es"),
            None,
            None,
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.headers[header::CONTENT_TYPE], "application/pdf");
    assert!(r.body.starts_with(b"%PDF-"));

    // Other users can read shared recipes but never modify them.
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{shared_id}"),
            None,
            Some(&bob),
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{private_id}"),
            None,
            Some(&bob),
        )
        .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    let r = app
        .call(
            Method::PUT,
            &format!("/api/recipes/{shared_id}"),
            Some(recipe_body("Hacked", true)),
            Some(&bob),
        )
        .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);
    let r = app
        .call(
            Method::DELETE,
            &format!("/api/recipes/{shared_id}"),
            None,
            Some(&bob),
        )
        .await;
    assert_eq!(r.status, StatusCode::FORBIDDEN);

    // Owner update replaces children.
    let mut body = recipe_body("Tortilla v2", false);
    body["steps"] = json!([{"stepNumber": 1, "instructions": "Just do it", "duration": 5}]);
    body["nutrition"] = Value::Null;
    let r = app
        .call(
            Method::PUT,
            &format!("/api/recipes/{shared_id}"),
            Some(body),
            Some(&ana),
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.json()["steps"].as_array().unwrap().len(), 1);
    assert_eq!(r.json()["nutrition"], Value::Null);
    assert_eq!(r.json()["category"], Value::Null);
    let r = app
        .call(Method::GET, "/api/recipes/shared", None, None)
        .await;
    assert_eq!(
        r.json()["totalElements"],
        0,
        "unshared recipes disappear from the feed"
    );

    // Deleting a category keeps the recipe.
    let mut body = recipe_body("With category", false);
    body["categoryId"] = cat["id"].clone();
    let rid = app
        .call(Method::POST, "/api/recipes", Some(body), Some(&ana))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();
    app.call(
        Method::DELETE,
        &format!("/api/categories/{}", cat["id"].as_str().unwrap()),
        None,
        Some(&ana),
    )
    .await;
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{rid}"),
            None,
            Some(&ana),
        )
        .await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(r.json()["category"], Value::Null);

    let r = app
        .call(
            Method::DELETE,
            &format!("/api/recipes/{rid}"),
            None,
            Some(&ana),
        )
        .await;
    assert_eq!(r.status, StatusCode::NO_CONTENT);
    let r = app
        .call(
            Method::GET,
            &format!("/api/recipes/{rid}"),
            None,
            Some(&ana),
        )
        .await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn guest_access_can_be_disabled() {
    let app = app_with(|c| c.guest_access = false).await;
    let (_, ana) = app.user("ana").await;
    let id = app
        .call(
            Method::POST,
            "/api/recipes",
            Some(recipe_body("Shared", true)),
            Some(&ana),
        )
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();

    assert_eq!(
        app.call(Method::GET, "/api/system/config", None, None)
            .await
            .json()["guestAccess"],
        false
    );
    assert_eq!(
        app.call(Method::GET, "/api/recipes/shared", None, None)
            .await
            .status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.call(Method::GET, &format!("/api/recipes/{id}"), None, None)
            .await
            .status,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        app.call(Method::GET, "/api/recipes/shared", None, Some(&ana))
            .await
            .status,
        StatusCode::OK
    );
}

#[tokio::test]
async fn rate_limit_on_credentials() {
    use axum::extract::ConnectInfo;
    use std::net::SocketAddr;

    let app = app_with(|c| c.auth.rate_limit_per_minute = 2).await;
    let peer: SocketAddr = "203.0.113.7:4000".parse().unwrap();
    let mut statuses = Vec::new();
    for _ in 0..3 {
        let mut req = Request::builder()
            .method(Method::POST)
            .uri("/api/auth/login")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(
                json!({"email": "x@example.com", "password": "whatever1"}).to_string(),
            ))
            .unwrap();
        req.extensions_mut().insert(ConnectInfo(peer));
        let res = app.router.clone().oneshot(req).await.unwrap();
        statuses.push(res.status());
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            assert!(res.headers().contains_key(header::RETRY_AFTER));
        }
    }
    assert_eq!(
        statuses,
        [
            StatusCode::UNAUTHORIZED,
            StatusCode::UNAUTHORIZED,
            StatusCode::TOO_MANY_REQUESTS
        ]
    );
}

#[tokio::test]
async fn cors_preflight() {
    let app = app().await;
    let req = Request::builder()
        .method(Method::OPTIONS)
        .uri("/api/auth/login")
        .header(header::ORIGIN, "http://localhost:5173")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
        .body(Body::empty())
        .unwrap();
    let res = app.router.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers()[header::ACCESS_CONTROL_ALLOW_ORIGIN],
        "http://localhost:5173"
    );
    assert_eq!(
        res.headers()[header::ACCESS_CONTROL_ALLOW_CREDENTIALS],
        "true"
    );
}

#[tokio::test]
async fn pdf_templates_and_page_flow() {
    let app = app().await;
    let (_, ana) = app.user("ana").await;

    let mut body = recipe_body("Tortilla de patatas", true);
    // A very long step must flow across pages.
    body["steps"][1]["instructions"] = json!("Remueve con cuidado. ".repeat(400));
    // Unreachable / forbidden images are skipped, not fatal.
    body["imageUrl"] = json!("http://127.0.0.1:1/nope.jpg");
    let id = app
        .call(Method::POST, "/api/recipes", Some(body), Some(&ana))
        .await
        .json()["id"]
        .as_str()
        .unwrap()
        .to_owned();

    for template in ["default-en", "default-es", "unknown"] {
        let r = app
            .call(
                Method::GET,
                &format!("/api/recipes/{id}/pdf/{template}"),
                None,
                Some(&ana),
            )
            .await;
        assert_eq!(r.status, StatusCode::OK);
        assert!(r.body.starts_with(b"%PDF-") && r.body.len() > 2000);
        assert_eq!(
            r.headers[header::CONTENT_DISPOSITION],
            format!("attachment; filename=\"recipe-{id}.pdf\"")
        );
    }
}
