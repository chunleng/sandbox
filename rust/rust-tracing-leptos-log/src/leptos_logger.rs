use leptos::logging::{error, log, warn};
use serde_json::{json, Map, Value};
use tracing::{event, Level};
use tracing_subscriber::Layer;

fn push_to_message(message: &mut String, k: &String, v: &Value) {
    if k == "message" {
        if let Some(s) = v.as_str() {
            message.push_str(s);
        }
    } else {
        message.push_str(&format!("{}={}", k, v));
    }
}

pub struct LeptosLoggingLayer;
impl<S> Layer<S> for LeptosLoggingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(&self, event: &event::Event<'_>, _ctx: tracing_subscriber::layer::Context<'_, S>) {
        let mut str_to_print = "".to_string();
        let mut visitor = MapVisitor(Map::new());
        event.record(&mut visitor);

        let mut items = visitor.0.iter();
        if let Some((k, v)) = items.next() {
            push_to_message(&mut str_to_print, k, v);
        }
        for (k, v) in items {
            str_to_print.push_str(", ");
            push_to_message(&mut str_to_print, k, v);
        }

        match event.metadata().level() {
            &Level::ERROR => {
                error!("{}", str_to_print);
            }
            &Level::WARN => {
                warn!("{}", str_to_print);
            }
            &Level::INFO | &Level::DEBUG | &Level::TRACE => {
                log!("{}", str_to_print);
            }
        }
    }
}

struct MapVisitor(Map<String, Value>);

impl tracing::field::Visit for MapVisitor {
    fn record_f64(&mut self, field: &tracing::field::Field, value: f64) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_u64(&mut self, field: &tracing::field::Field, value: u64) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_i64(&mut self, field: &tracing::field::Field, value: i64) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_i128(&mut self, field: &tracing::field::Field, value: i128) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_u128(&mut self, field: &tracing::field::Field, value: u128) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_bool(&mut self, field: &tracing::field::Field, value: bool) {
        self.0.insert(field.name().to_string(), json!(value));
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0
            .insert(field.name().to_string(), json!(format!("{:?}", value)));
    }
}
