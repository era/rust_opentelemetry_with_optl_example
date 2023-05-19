use actix_web::{get, App, HttpServer, Responder};
use opentelemetry_api::{global, metrics, Context, KeyValue};
use opentelemetry_otlp::{ExportConfig, WithExportConfig};
use opentelemetry_sdk::{metrics::MeterProvider, runtime};

fn init_meter_provider() -> metrics::Result<MeterProvider> {
    let export_config = ExportConfig {
        endpoint: "http://localhost:4317".to_string(),
        ..ExportConfig::default()
    };
    opentelemetry_otlp::new_pipeline()
        .metrics(runtime::Tokio)
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_export_config(export_config),
        )
        .build()
}

fn inc_request_counter() {
    let cx = Context::current();
    let meter = global::meter("hello_world_saas");
    let counter = meter.u64_counter("requests").init();
    counter.add(&cx, 1, &[KeyValue::new("server", "my_pc")]);
}

#[get("/")]
async fn index() -> impl Responder {
    inc_request_counter();
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let _ = init_meter_provider().unwrap();

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
