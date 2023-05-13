# Complex GitHub Usage Function with Rust and Tokio

This function demonstrates a more complex usage of the `github_async` function, which retrieves an access token from GitHub using OAuth2. It showcases how to interact with the GitHub API to list public repositories and retrieve user email addresses using Rust and the Tokio async runtime.

## Rust, Tokio, and API Interaction

Rust is a systems programming language that emphasizes safety, concurrency, and performance. It is well-suited for building high-performance and reliable applications, such as web services and API clients.

Tokio is an asynchronous runtime for Rust that allows you to write asynchronous code using the `async`/`await` syntax. It is built on top of Rust's `async-std` library and provides an event-driven, non-blocking I/O model for efficient execution of tasks.

Interacting with API services in Rust typically involves making HTTP requests using libraries such as `reqwest` or `hyper`. The OAuth2 process, which requires exchanging authorization codes and tokens, can be simplified using the `oauth2` crate.

In this example, we use the `oauth2` crate along with the `reqwest` HTTP client and the Tokio async runtime to interact with the GitHub API.

## Usage

1. First, make sure you have set the `GITHUB_CLIENT_ID` and `GITHUB_CLIENT_SECRET` environment variables.
2. Run the program with `cargo run`.
3. Follow the instructions to open the provided URL in your browser.
4. After authorizing the application, you'll be redirected to a local server. The access token will be displayed in the terminal.
5. The function will use the access token to call the GitHub API, listing public repositories and retrieving the user's email address.

## Example

Here's an example of how to call the `complex_github_usage` function:

```rust
pub async fn complex_github_usage() {
    let token = github_async().await.unwrap();

    let public_repos = get_public_repos(&token).await.unwrap();
    let email = get_user_email(&token).await.unwrap();

    println!("Public repositories:");
    for repo in public_repos {
        println!("- {}", repo);
    }

    println!("\nUser email: {}", email);
}

#[tokio::main]
async fn main() {
    complex_github_usage().await;
}
