use crate::env::model::Config;
use crate::firebs::model::{ChannelSet, DataSet};
use crate::openai::model::OpenAIResult;
use crate::server::model::Prompt;
use firestore::*;

pub async fn connect(cfg: &Config) -> FirestoreResult<FirestoreDb> {
    let db = FirestoreDb::with_options_service_account_key_file(
        FirestoreDbOptions::new(cfg.firebase_project.to_string()),
        cfg.firebase_key_file.clone().into(),
    )
    .await?;

    Ok(db)
}

pub async fn insert(
    db: FirestoreResult<FirestoreDb>,
    params: &Prompt,
    result: &OpenAIResult,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let channels = ChannelSet::new(
        Some("c1".to_string()),
        Some("c2".to_string()),
        Some("c3".to_string()),
        Some("c4".to_string()),
    );

    let s = DataSet::new(
        params.id.clone(),
        params.prompt.clone(),
        result.clone(),
        "https://placehold.co/400x400".to_string(),
        channels,
    );

    let _o: DataSet = db?
        .fluent()
        .insert()
        .into("completion")
        .document_id(&s.id)
        .object(&s)
        .execute()
        .await?;

    Ok(())
}