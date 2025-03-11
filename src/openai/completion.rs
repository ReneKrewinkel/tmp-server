use openai_api_rust::{Auth, Message, OpenAI, Role};
use openai_api_rust::chat::*;
use crate::server::model::Prompt;
use crate::env::model::Config;
use super::model::OpenAIResult;

pub fn openai_completion(params: &Prompt, cfg: &Config) -> OpenAIResult {

    let auth = Auth::new(&cfg.api_token);
    let openai = OpenAI::new(auth, &cfg.api_url);
    let body = ChatBody {
        model: cfg.api_model.clone(),
        max_tokens: Some(200),
        temperature: Some(0_f32),
        top_p: Some(0_f32),
        n: Some(2),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![Message { role: Role::User, content: params.prompt.clone() }],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();

    OpenAIResult::new( params.id.clone(), message.content.clone() )

}