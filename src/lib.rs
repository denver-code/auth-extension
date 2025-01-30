use axum::{response::Json, routing::get, Router};
use corex_api::ExtensionTrait;
use serde_json::json;

pub struct AuthExtension;

impl ExtensionTrait for AuthExtension {
    fn name(&self) -> &'static str {
        "AuthExtension"
    }

    fn extend(&self, router: Router) -> Router {
        router.route(
            "/auth",
            get(|| async { Json(json!({ "message": "Auth endpoint" })) }),
        )
    }
}
