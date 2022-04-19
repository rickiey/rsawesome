// use opentelemetry::sdk::export::trace::stdout;
use tracing::{error, info, span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn use_opentelemetry() {
    // Install a new OpenTelemetry trace pipeline
// Create a new OpenTelemetry pipeline
//     let tracer = stdout::new_pipeline().install_simple();

    let tracer = opentelemetry_jaeger::new_pipeline()
        .with_agent_endpoint("localhost:6831")
        .with_service_name("report_example")
        .install_simple().unwrap();
// Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

// Use the tracing subscriber `Registry`, or any other subscriber
// that impls `LookupSpan`
    let subscriber = Registry::default().with(telemetry);

// Trace executed code
    tracing::subscriber::with_default(subscriber, || {
        // Spans will be sent to the configured OpenTelemetry exporter
        let root = span!(tracing::Level::TRACE, "app_start", work_units = 2);

        let _enter = root.enter();


        let s = 45354;
        info!(m ="as", %s);
        error!("This event will be logged in the root span.");
    });
}