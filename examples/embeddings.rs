use mistralai_client::v1::{client::Client, constants::EmbedModel};

fn main() {
    // This example suppose you have set the `MISTRAL_API_KEY` environment variable.
    let client: Client = Client::new(None, None, None, None).unwrap();

    let model = EmbedModel::MistralEmbed;
    let input = ["Embed this sentence.", "As well as this one."]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let options = None;

    let response = client.embeddings(model, input, options).unwrap();
    println!("First Embedding: {:?}", response.data[0]);
    // => "First Embedding: {...}"
}
