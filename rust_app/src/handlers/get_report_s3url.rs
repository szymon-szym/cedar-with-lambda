use authorization::models::{ActionVerb, AuthorizationDecision, OwnerId, ResourceType, UserId};
use http::header::AUTHORIZATION;
use lambda_http::{run, service_fn, tracing};
use lambda_http::{Body, Error, Request, RequestExt, Response};
use reports::service::ReportsService;

#[path = "../authorization/mod.rs"]
pub mod authorization;

#[path = "../user_context/mod.rs"]
pub mod user_context;

#[path = "../reports/mod.rs"]
pub mod reports;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let reports_service = reports::service::ReportsService::new();

    run(service_fn(|ev| function_handler(ev, &reports_service))).await
}

pub(crate) async fn function_handler(
    event: Request,
    reports_service: &ReportsService,
) -> Result<Response<Body>, Error> {

    // get user id from auth token
    let token = event
        .headers()
        .get(AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|token| token.strip_prefix("Bearer "))
        .ok_or("Missing auth token")?;

    let user_id = user_context::jwt::get_user_id(token)?;

    // get report id

    let report_id = event
        .query_string_parameters_ref()
        .and_then(|params| params.first("report"))
        .ok_or("Missing report id")?;


    // get report

    let report = reports_service.get_report(report_id.to_string())
        .ok_or("missing report with given id")?;

    // authorize request
    let decision = authorization::service::authorize(
        UserId(user_id),
        ActionVerb::GenerateS3Url,
        ResourceType::S3Object,
        OwnerId(report.owner_id),
    )?;


    // return response

    let resp = match decision {
        AuthorizationDecision::Allow => Response::builder()
            .status(200)
            .header("content-type", "text/html")
            .body("Here you go, this is your dummy s3 url: dummy".into())
            .map_err(Box::new)?,
        AuthorizationDecision::Deny => Response::builder()
            .status(404)
            .header("content-type", "text/html")
            .body("You are not authorized".into())
            .map_err(Box::new)?,
    };
    Ok(resp)
}
