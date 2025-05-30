use jrest::expect;
use mistralai_client::v1::{client::Client, error::ClientError};

#[derive(Debug)]
struct _Foo {
    _client: Client,
}

#[test]
fn test_client_new_with_none_params() {
    let maybe_original_mistral_api_key = std::env::var("MISTRAL_API_KEY").ok();
    std::env::remove_var("MISTRAL_API_KEY");
    std::env::set_var("MISTRAL_API_KEY", "test_api_key_from_env");

    let client = Client::new(None, None, None, None).unwrap();

    expect!(client.api_key).to_be("test_api_key_from_env".to_string());
    expect!(client.endpoint).to_be("https://api.mistral.ai/v1".to_string());
    expect!(client.max_retries).to_be(5);
    expect!(client.timeout).to_be(120);

    match maybe_original_mistral_api_key {
        Some(original_mistral_api_key) => {
            std::env::set_var("MISTRAL_API_KEY", original_mistral_api_key)
        }
        None => std::env::remove_var("MISTRAL_API_KEY"),
    }
}

#[test]
fn test_client_new_with_all_params() {
    let maybe_original_mistral_api_key = std::env::var("MISTRAL_API_KEY").ok();
    std::env::remove_var("MISTRAL_API_KEY");

    let api_key = "test_api_key_from_param".to_owned();
    let endpoint = "https://example.org".to_owned();
    let max_retries = 10;
    let timeout = 20;

    let client = Client::new(
        Some(api_key.clone()),
        Some(endpoint.clone()),
        Some(max_retries),
        Some(timeout),
    )
    .unwrap();

    expect!(client.api_key).to_be(api_key);
    expect!(client.endpoint).to_be(endpoint);
    expect!(client.max_retries).to_be(max_retries);
    expect!(client.timeout).to_be(timeout);

    match maybe_original_mistral_api_key {
        Some(original_mistral_api_key) => {
            std::env::set_var("MISTRAL_API_KEY", original_mistral_api_key)
        }
        None => std::env::remove_var("MISTRAL_API_KEY"),
    }
}

#[test]
fn test_client_new_with_api_key_as_both_env_and_param() {
    let maybe_original_mistral_api_key = std::env::var("MISTRAL_API_KEY").ok();
    std::env::remove_var("MISTRAL_API_KEY");
    std::env::set_var("MISTRAL_API_KEY", "test_api_key_from_env");

    let api_key = "test_api_key_from_param".to_owned();
    let endpoint = "https://example.org".to_owned();
    let max_retries = 10;
    let timeout = 20;

    let client = Client::new(
        Some(api_key.clone()),
        Some(endpoint.clone()),
        Some(max_retries),
        Some(timeout),
    )
    .unwrap();

    expect!(client.api_key).to_be(api_key);
    expect!(client.endpoint).to_be(endpoint);
    expect!(client.max_retries).to_be(max_retries);
    expect!(client.timeout).to_be(timeout);

    match maybe_original_mistral_api_key {
        Some(original_mistral_api_key) => {
            std::env::set_var("MISTRAL_API_KEY", original_mistral_api_key)
        }
        None => std::env::remove_var("MISTRAL_API_KEY"),
    }
}

#[test]
fn test_client_new_with_missing_api_key() {
    let maybe_original_mistral_api_key = std::env::var("MISTRAL_API_KEY").ok();
    std::env::remove_var("MISTRAL_API_KEY");

    let call = || Client::new(None, None, None, None);

    match call() {
        Ok(_) => panic!("Expected `ClientError::MissingApiKey` but got Ok.`"),
        Err(error) => assert_eq!(error, ClientError::MissingApiKey),
    }

    match maybe_original_mistral_api_key {
        Some(original_mistral_api_key) => {
            std::env::set_var("MISTRAL_API_KEY", original_mistral_api_key)
        }
        None => std::env::remove_var("MISTRAL_API_KEY"),
    }
}
