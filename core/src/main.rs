#[macro_use]
extern crate rocket;
mod consul;
use rocket::response::status::BadRequest;
use rocket::response::stream::ReaderStream;
use rocket::serde::json::Json;
use rs_consul::Consul;
use serde::Deserialize;
use std::process::Stdio;
use tokio::process::{ChildStdout, Command};

#[get("/")]
fn index() -> &'static str {
    "Hello, world! The client is up and running"
}
#[derive(Deserialize)]
struct ServiceConfig {
    image: String,
    arguments: Vec<String>,
}

#[post("/services/new", data = "<config>")]
async fn new_service(config: Json<ServiceConfig>) -> Result<String, BadRequest<String>> {
    let output = Command::new("podman")
        .args(["run", "-d"])
        .args(&config.arguments)
        .arg(&config.image)
        .output()
        .await
        .unwrap();

    if output.status.success() {
        let id = String::from_utf8_lossy(&output.stdout);
        println!("Started service {} with id {}", config.image, &id);
        Ok(id.to_string())
    } else {
        let s = format!(
            "Error: {} {} ",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        println!("{s}");
        Err(BadRequest(Some(s)))
    }
}

#[get("/services/logs/<id>")]
fn stream_logs(id: &str) -> ReaderStream![ChildStdout] {
    let mut child = Command::new("podman")
        .args(["logs", "-f", id])
        .stdout(Stdio::piped())
        .spawn()
        .expect("faild to spawn log process");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    ReaderStream::one(stdout)
}

#[launch]
fn rocket() -> _ {
    let config = Config {
        
    }
    let consul = Consul::new(config)
    rocket::build().mount("/", routes![index, new_service, stream_logs])
}

// stop docker container

// register itself to consul
