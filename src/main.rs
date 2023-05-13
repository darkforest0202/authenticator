use tokio;
mod github_auth;
use github_auth::github_getter;
mod google;
mod google_devicecode;
mod letterboxd;


#[tokio::main]
async fn main() {
    github_getter().await;
    google_devicecode::google_device();
    letterboxd::run_letterboxd();
    google::google();
}
