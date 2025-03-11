use super::model::Prompt;
use crate::env::config;
use crate::firebs::database;
use crate::openai::completion;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/prompt")]
async fn submit_prompt(params: web::Json<Prompt>) -> impl Responder {
    let cfg = config::read_config();
    let db = database::connect(&cfg).await;
    let completion = completion::openai_completion(&params, &cfg);
    let _b = database::insert(db, &params, &completion).await;

    HttpResponse::Ok().json(completion.clone())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(submit_prompt); //.service(...)
}
