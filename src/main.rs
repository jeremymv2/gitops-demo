#[macro_use]
extern crate rocket;

use once_cell::sync::Lazy;
use prometheus::{opts, IntCounterVec};
use rocket_prometheus::PrometheusMetrics;

const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

static NAME_COUNTER: Lazy<IntCounterVec> = Lazy::new(|| {
    IntCounterVec::new(opts!("name_counter", "Count of names"), &["name"])
        .expect("Could not create lazy IntCounterVec")
});

mod routes {
    use rocket::serde::json::Json;
    use serde::Deserialize;
    use std::env;

    use super::{CARGO_PKG_VERSION, NAME_COUNTER};

    #[get("/hello/<name>?<caps>")]
    pub fn hello(name: &str, caps: Option<bool>) -> String {
        let env_version_name = "APP_VERSION";
        let mut app_version = None;

        match env::var(env_version_name) {
            Ok(v) => {
                println!("Found version {} from {} ENVVAR", v, env_version_name);
                app_version = Some(v)
            }
            Err(_) => {
                println!("Using version from Cargo.toml");
                app_version = Some(CARGO_PKG_VERSION.to_string())
            }
        }
        let name = caps
            .unwrap_or_default()
            .then(|| name.to_uppercase())
            .unwrap_or_else(|| name.to_string());
        NAME_COUNTER.with_label_values(&[&name]).inc();
        format!(
            "Hello, {}! I am running version: v{}",
            name,
            app_version.unwrap_or_else(|| "n/a".to_string())
        )
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
    println!("Cargo Pkg Version: {}", CARGO_PKG_VERSION);
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
