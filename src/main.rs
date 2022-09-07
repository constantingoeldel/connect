use std::process::Stdio;

use rocket::response::status::BadRequest;
use rocket::response::stream::{self, ReaderStream};
use rocket::response::stream::{stream, TextStream};
use rocket::serde::json::Json;
use serde::Deserialize;
use tokio::io::{AsyncBufReadExt, BufReader, BufStream};
use tokio::process::{ChildStdout, Command};
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
#[derive(Deserialize)]
struct ServiceConfig {
    image: String,
}

#[post("/services/new", data = "<config>")]
async fn new_service(config: Json<ServiceConfig>) -> Result<String, BadRequest<String>> {
    let output = Command::new("podman")
        .args(["run", "-d", &config.image])
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
        .stderr(Stdio::piped())
        .spawn()
        .expect("faild to spawn log process");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let stderr = child
        .stderr
        .take()
        .expect("child did not have a handle to stderr");

    ReaderStream::one(stdout)

    // if output.status.success() {

    // } else {
    //     let s = format!(
    //         "Error: {} {} ",
    //         String::from_utf8_lossy(&output.stdout),
    //         String::from_utf8_lossy(&output.stderr)
    //     );
    //     println!("{s}");
    //     Err(BadRequest(Some(s)))
    // }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, new_service, stream_logs])
}

// start docker container from specific image

// stop docker container

// log onto tailscale network

// connect to cloudflare

// raft consensus about state

// raft leader delegates tasks
