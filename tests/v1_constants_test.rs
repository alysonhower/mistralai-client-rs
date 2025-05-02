use jrest::expect;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatParams},
    client::Client,
    constants::Model,
};

#[test]
fn test_model_constant() {
    let models = vec![
        Model::Ministral3b,
        Model::Ministral8b,
        Model::MistralSmall,
        Model::OpenMistralNemo,
        Model::Codestral,
        Model::MistralSaba,
        Model::MistralLarge,
        Model::PixtralLarge,
    ];

    let client = Client::new(None, None, None, None).unwrap();

    let messages = vec![ChatMessage::new_user_message("A number between 0 and 100?")];
    let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(42),
        ..Default::default()
    };

    for model in models {
        let response = client
            .chat(model.clone(), messages.clone(), Some(options.clone()))
            .unwrap();

        expect!(response.model).to_be(model);
        expect!(response.object).to_be("chat.completion".to_string());
        expect!(response.choices.len()).to_be(1);
        expect!(response.choices[0].index).to_be(0);
        expect!(response.choices[0].message.content.len()).to_be_greater_than(0);
    }
}
