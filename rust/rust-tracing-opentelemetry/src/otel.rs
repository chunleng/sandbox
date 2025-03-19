use opentelemetry::{KeyValue, trace::TraceResult};
use opentelemetry_otlp::{WithExportConfig, new_exporter, new_pipeline};
use opentelemetry_sdk::{
    Resource,
    runtime::Tokio,
    trace::{Config, Sampler},
};
use opentelemetry_semantic_conventions::{SCHEMA_URL, resource::SERVICE_NAME};

pub struct OtelHandler {
    pub provider: opentelemetry_sdk::trace::TracerProvider,
}

impl OtelHandler {
    pub fn init_tracer(sample_rate: f64) -> TraceResult<Self> {
        Ok(Self {
            provider: new_pipeline()
                .tracing()
                .with_exporter(
                    new_exporter()
                        .tonic()
                        .with_endpoint("http://localhost:4317"),
                )
                .with_trace_config(
                    Config::default()
                        .with_resource(Resource::from_schema_url(
                            [KeyValue::new(SERVICE_NAME, "rust-sandbox")],
                            SCHEMA_URL,
                        ))
                        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
                            sample_rate,
                        )))),
                )
                .install_batch(Tokio)?,
        })
    }

    pub fn shutdown_tracer(&self) -> TraceResult<()> {
        self.provider.shutdown()
    }
}
