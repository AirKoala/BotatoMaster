use eyre::{Result, WrapErr};
use sqlx::SqlitePool;

use crate::message_matcher::responder::SimpleResponder;

pub struct ResponseModel {
    id: i64,
    response: String,
    bias: i64,
    content_matcher_id: i64,
}

pub async fn fetch_responses_for_matcher(
    matcher_id: i64,
    pool: &SqlitePool,
) -> Result<Vec<ResponseModel>> {
    Ok(sqlx::query_as!(
        ResponseModel,
        "SELECT * FROM response
        WHERE content_matcher_id = $1",
        matcher_id
    )
    .fetch_all(pool)
    .await
    .wrap_err("Failed to fetch responses for matcher.")?)
}

pub fn simple_responder_from_models(models: Vec<ResponseModel>) -> SimpleResponder {
    SimpleResponder::from_tuples(
        models
            .into_iter()
            .map(|model| (model.response, model.bias as u64))
            .collect(),
    )
}
