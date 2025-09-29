use std::{
    io::{Error, ErrorKind},
    str::FromStr,
};

use serde::Serialize;
use warp::{
    Filter,
    http::StatusCode,
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
struct QuestionId(String);

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.is_empty() {
            false => Ok(Self(s.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

#[derive(Debug)]
struct InvalidId;

impl Reject for InvalidId {}

#[tokio::main]
async fn main() {
    let get_items = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and_then(get_questions)
        .recover(return_error);

    let routes = get_items;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn get_questions() -> Result<impl Reply, Rejection> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First question".to_string(),
        "Content of question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    match question.id.0.parse::<i32>() {
        Err(_) => Err(warp::reject::custom(InvalidId)),
        Ok(_) => Ok(warp::reply::json(&question)),
    }
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(_) = r.find::<InvalidId>() {
        Ok(warp::reply::with_status(
            "No valid ID presented",
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found",
            StatusCode::NOT_FOUND,
        ))
    }
}
