use spin_sdk::http::{IntoResponse, Params, Request, Router, Response};
use spin_sdk::{http_component};
use spin_sdk::key_value::{Store};
use serde::Serialize;


static COUNTER_KEY: &str = "counter";

/// A simple Spin HTTP component.
#[http_component]
fn handle_mdn_aoc2023_c1(req: Request) -> Response {
    println!("Handling request to {:?}", req.header("spin-full-url"));

    let mut router = Router::new();

    router.get("/", handle_index);
    router.get("/data", handle_get_data);
    router.get("/counter", handle_counter);

    router.handle(req)
}

fn handle_index(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    Ok(http::Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")?)
}

fn handle_get_data(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let response = serde_json::to_string(&Data::new("Hello, Fermyon".to_string()));

    Ok(http::Response::builder()
        .status(200)
        //.header("content-type", "text/plain")
        .body(response.unwrap())?)
}


fn handle_counter(_req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {

    let store = Store::open_default()?;
    let mut counter = Counter {
        value: match store.get(COUNTER_KEY)? {
            None => 0,
            Some(bytes) => {
                i32::from_le_bytes(bytes.try_into().unwrap())
            }
        }
    };

    counter.value += 1;

    store.set(COUNTER_KEY, &counter.value.to_le_bytes())?;
    // let counter = Counter {
    //     value: 1
    // };
    let response = serde_json::to_string(&counter)?;

    Ok(http::Response::builder()
        .status(200)
        //.header("content-type", "text/plain")
        .body(response)?)
    // let response = serde_json::to_string(&Data::new("Hello, Fermyon".to_string()));
    //
    // Ok(http::Response::builder()
    //     .status(200)
    //     //.header("content-type", "text/plain")
    //     .body(response.unwrap())?)
}

#[derive(Debug, Serialize)]
struct Counter {
    pub value: i32,
}

#[derive(Debug, Serialize)]
struct Data {
    pub value: String,
}

impl Data {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}