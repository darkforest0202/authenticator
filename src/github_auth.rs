//!
//! This example showcases the Github OAuth2 process for requesting access to the user's public repos and
//! email address.
//!
//! Before running it, you'll need to generate your own Github OAuth2 credentials.
//!
//! In order to run the example call:
//!
//! ```sh
//! GITHUB_CLIENT_ID=xxx GITHUB_CLIENT_SECRET=yyy cargo run --example github
//! ```
//!
//! ...and follow the instructions.
//!

use oauth2::basic::BasicClient;

// Alternatively, this can be `oauth2::curl::http_client` or a custom client.
use oauth2::reqwest::async_http_client;
use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl,
    RequestTokenError, Scope, TokenResponse, TokenUrl,
};
use std::env;
use std::error::Error;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
use url::Url;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    html_url: String,
}

#[derive(Deserialize, Debug)]
struct User {
    name: String,
    email: Option<String>,
}

pub async fn github_getter() {
    let token_result = github_async().await;

    match token_result {
        Ok(token) => {
            // Set up a new reqwest client
            let client = reqwest::Client::new();

            // Get the user's GitHub details
            let user_url = "https://api.github.com/user";
            let user_response = client
                .get(user_url)
                .bearer_auth(token.secret()) // Use token.secret() instead of token.clone()
                .send()
                .await
                .expect("Failed to send request");

            if user_response.status().is_success() {
                let user: serde_json::Value = user_response
                    .json()
                    .await
                    .expect("Failed to parse user response");

                println!("User Details: {:#?}", user);
            } else {
                eprintln!("Failed to fetch user details");
            }

            // Get the user's public repositories
            let repos_url = "https://api.github.com/user/repos";
            let repos_response = client
                .get(repos_url)
                .bearer_auth(token.secret()) // Use token.secret() instead of token.clone()
                .send()
                .await
                .expect("Failed to send request");

            if repos_response.status().is_success() {
                let repos: Vec<serde_json::Value> = repos_response
                    .json()
                    .await
                    .expect("Failed to parse repositories response");

                println!("User Repositories: {:#?}", repos);
            } else {
                eprintln!("Failed to fetch user repositories");
            }
        }
        Err(e) => {
            eprintln!("Error obtaining access token: {}", e);
        }
    }
}


async fn github_async() -> Result<AccessToken, Box<dyn Error>> {
    let github_client_id = ClientId::new(
        env::var("GITHUB_CLIENT_ID").expect("Missing the GITHUB_CLIENT_ID environment variable."),
    );
    let github_client_secret = ClientSecret::new(
        env::var("GITHUB_CLIENT_SECRET")
            .expect("Missing the GITHUB_CLIENT_SECRET environment variable."),
    );
    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    // Set up the config for the Github OAuth2 process.
    let client = BasicClient::new(
        github_client_id,
        Some(github_client_secret),
        auth_url,
        Some(token_url),
    )
    // This example will be running its own server at localhost:8080.
    // See below for the server implementation.
    .set_redirect_uri(
        RedirectUrl::new("http://localhost:8080".to_string()).expect("Invalid redirect URL"),
    );

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("public_repo".to_string()))
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
    );

    // A very naive implementation of the redirect server.
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();

    if let Ok((mut stream, _)) = listener.accept().await {
        let code;
        let state;
        {
            let mut reader = BufReader::new(&mut stream);

            let mut request_line = String::new();
            reader.read_line(&mut request_line).await.unwrap();

            let redirect_url = request_line.split_whitespace().nth(1).unwrap();
            let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

            let code_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                })
                .unwrap();

            let (_, value) = code_pair;
            code = AuthorizationCode::new(value.into_owned());

            let state_pair = url
                .query_pairs()
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "state"
                })
                .unwrap();

            let (_, value) = state_pair;
            state = CsrfToken::new(value.into_owned());
        }

        let message = "Go back to your terminal :)";
        let response = format!(
            "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
            message.len(),
            message
        );
        stream.write_all(response.as_bytes()).await.unwrap();

        println!("Github returned the following code:\n{}\n", code.secret());
        println!(
            "Github returned the following state:\n{} (expected `{}`)\n",
            state.secret(),
            csrf_state.secret()
        );

        let token_res = client
            .exchange_code(code)
            .request_async(async_http_client)
            .await;

        match token_res {
            Ok(token) => {
                // ... (El código de impresión de token y alcance se mantiene igual)

                // Devuelve el token en lugar de imprimirlo.
                Ok(token.access_token().clone())
            }
            Err(e) => {
                eprintln!("Error during token exchange: {:?}", e);
                Err(Box::new(e))
            }
        }
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to accept connection",
        )))
    }
}
