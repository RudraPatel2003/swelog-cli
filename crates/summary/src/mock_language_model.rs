use async_trait::async_trait;
use llm::language_model::LanguageModel;
use miette::Result;
use mockall::mock;

mock! {
    pub LanguageModel {}

    #[async_trait]
    impl LanguageModel for LanguageModel {
        async fn generate_response(&self, prompt: &str) -> Result<String>;
    }
}

pub fn get_prompt_echoing_language_model() -> MockLanguageModel {
    let mut language_model = MockLanguageModel::new();

    language_model
        .expect_generate_response()
        .times(1)
        .returning(|prompt| Ok(format!("generated from prompt:\n{prompt}")));

    language_model
}

pub fn get_language_model_responding_with(response: String) -> MockLanguageModel {
    let mut language_model = MockLanguageModel::new();

    language_model.expect_generate_response().times(1).return_once(move |_| Ok(response));

    language_model
}

pub fn get_uncalled_language_model() -> MockLanguageModel {
    let mut language_model = MockLanguageModel::new();

    language_model.expect_generate_response().never();

    language_model
}
