use mistralai_client::v1::{client::Client, files};

#[tokio::main]
async fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client = Client::new(None, None, None, None).unwrap();
    let result = client
        .upload(
            "/home/a/repos/rust/mistralai-client-rs/data/example.png",
            files::FilePurpose::Ocr,
        )
        .await;

    println!("{:?}", result)
}
