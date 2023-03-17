use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::cost_items;

#[derive(OpenApi)]
#[openapi(
    paths(
        cost_items::find_all,
        cost_items::filter,
        cost_items::find,
        cost_items::create,
        cost_items::update,
        cost_items::delete
    ),
    components(schemas(cost_items::CostItems))
)]
pub struct ApiDoc;

pub fn init_swagger(config: &mut web::ServiceConfig) {
    let openapi = ApiDoc::openapi();
    config.service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", openapi));
}