#[macro_use]
extern crate rocket;
// fixing

use once_cell::sync::Lazy;
use prometheus::{opts, IntCounterVec};
use rocket_prometheus::PrometheusMetrics;

const VERSION: &str = env!("CARGO_PKG_VERSION");

static NAME_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("name_counter", "Count of names"), &["name"])
        .expect("Could not create lazy IntCounterVec")
});

mod routes {
    use rocket::serde::json::Json;
    use serde::Deserialize;

    use super::{NAME_COUNTER, VERSION};

    #[get("/hello/<name>?<caps>")]
    pub fn hello(name: &str, caps: Option<bool>) -> String {
        let name = caps
            .unwrap_or_default()
            .then(|| name.to_uppercase())
            .unwrap_or_else(|| name.to_string());
        NAME_COUNTER.with_label_values(&[&name]).inc();
        format!("Hello, {}! I am running version: v{}", name, VERSION)
    }

    #[derive(Deserialize)]
    pub struct Person {
        age: u8,
    }

    #[post("/hello/<name>?<caps>", format = "json", data = "<person>")]
    pub fn hello_post(name: String, person: Json<Person>, caps: Option<bool>) -> String {
        let name = caps
            .unwrap_or_default()
            .then(|| name.to_uppercase())
            .unwrap_or_else(|| name.to_string());
        NAME_COUNTER.with_label_values(&[&name]).inc();
        format!("Hello, {} year old named {}!", person.age, name)
    }
}

#[launch]
fn rocket() -> _ {
    println!("Running version: {}", VERSION);
    let prometheus = PrometheusMetrics::new();
    prometheus
        .registry()
        .register(Box::new(NAME_COUNTER.clone()))
        .unwrap();

    rocket::build()
        .attach(prometheus.clone())
        .mount("/", routes![routes::hello, routes::hello_post])
        .mount("/metrics", prometheus)
}
