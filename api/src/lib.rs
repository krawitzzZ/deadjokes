mod dto;
mod error_handler;
mod query;
mod response;
mod router;

use actix_cors::Cors;
use actix_files as fs;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::middleware::{DefaultHeaders, ErrorHandlers};
use actix_web::{http, web, App, Error, HttpServer};
use actix_web_lab::middleware::{CatchPanic, NormalizePath, PanicReporter};
use anyhow::Context;
use tracing::Span;
use tracing_actix_web::{DefaultRootSpanBuilder, RootSpanBuilder, TracingLogger};

use app::{AppResult, AppState};
use config::ApiConfig;

use crate::error_handler::{handle_data, handle_generic};

pub struct DeadjokesApiRootSpanBuilder;

impl RootSpanBuilder for DeadjokesApiRootSpanBuilder {
    fn on_request_start(request: &ServiceRequest) -> Span {
        tracing_actix_web::root_span!(request, user_id = tracing::field::Empty)
    }

    fn on_request_end<B: MessageBody>(span: Span, outcome: &Result<ServiceResponse<B>, Error>) {
        DefaultRootSpanBuilder::on_request_end(span, outcome);
    }
}

#[actix_web::main]
async fn start(config: ApiConfig, state: AppState) -> AppResult<()> {
    let port = config.port();
    let static_dir = config.static_dir().to_path_buf();
    let app_state = web::Data::new(state);

    infra::tracing::init(app_state.app_name(), &config);

    tracing::info!(port, "starting deadjokes-api server on port: {port}");

    HttpServer::new(move || {
        let json = web::JsonConfig::default().limit(1024 * 1024 * 10);
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
        App::new()
            // Util middleware
            .wrap(NormalizePath::trim())
            .wrap(cors)
            // Error handling
            .app_data(json.error_handler(|err, _| handle_data(err)))
            .app_data(web::QueryConfig::default().error_handler(|err, _| handle_data(err)))
            .app_data(web::PathConfig::default().error_handler(|err, _| handle_data(err)))
            .wrap(ErrorHandlers::new().default_handler(handle_generic))
            .wrap(DefaultHeaders::new().add(http::header::ContentType::json()))
            .wrap(CatchPanic::default())
            .wrap(PanicReporter::new(|err| {
                match err.downcast_ref::<String>() {
                    Some(error) => tracing::error!(error, "panic during request processing"),
                    None => tracing::error!("panic during request processing"),
                }
            }))
            .wrap(TracingLogger::<DeadjokesApiRootSpanBuilder>::new())
            // Application related data setup
            .app_data(app_state.clone())
            .service(
                fs::Files::new("/static", static_dir.clone())
                    .use_last_modified(true)
                    .prefer_utf8(true),
            )
            .service(web::scope("/api").configure(router::init))
    })
    .bind(("0.0.0.0", port))
    .context(format!("failed to bind server to port `{port}`"))?
    .run()
    .await
    .context(format!("unexpected error while running the server"))?;

    Ok(())
}

pub fn main(config: ApiConfig, state: AppState) {
    let result = start(config, state);

    if let Some(e) = result.err() {
        tracing::error!(
            error = %e,
            "unexpected error occurred during server runtime"
        );
    }
}
