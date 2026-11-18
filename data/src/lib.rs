//! Flashcards' data source: answers the sources `app.contract` declares,
//! in place of `app.ts`.

#![deny(missing_docs)]

use exact_plan::Value;
use exact_runner::{Answer, DataError, DataSource, Outcome, Request, Store};
use serde::Deserialize;

/// The flashcards server.
const API: &str = "http://localhost:3000";

/// The app's data source.
#[derive(Default, Clone, Copy)]
pub struct FlashcardData;

#[derive(Deserialize)]
struct Page {
  data: Vec<Flashcard>,
}

#[derive(Deserialize)]
struct Flashcard {
  id: f64,
  title: String,
  content: String,
}

impl Flashcard {
  fn to_value(&self) -> Value {
    Value::record(vec![
      Value::Number(self.id),
      Value::str(&self.title),
      Value::str(&self.content),
    ])
  }
}

impl DataSource for FlashcardData {
  fn query(&mut self, source: &str, _args: &[Value]) -> Result<Value, DataError> {
    match source {
      "loadFlashcards" => Err(DataError::DeferredAtBake {
        message: "flashcards are fetched at launch".into(),
        warning: false,
      }),
      _ => Err(DataError::UnknownSource(source.into())),
    }
  }

  fn answer(&mut self, _: &mut Store, source: &str, args: &[Value]) -> Result<Answer, DataError> {
    match source {
      "loadFlashcards" => Ok(Answer::Later(Request::get(&format!("{API}/flashcards")))),
      _ => self.query(source, args).map(Answer::Now),
    }
  }

  fn parse(
    &mut self,
    _: &mut Store,
    source: &str,
    _: &[Value],
    outcome: Outcome,
  ) -> Result<Answer, DataError> {
    match (source, outcome) {
      ("loadFlashcards", Outcome::Response(response)) => {
        let page: Page = serde_json::from_slice(&response.body).unwrap_or(Page { data: vec![] });
        Ok(Answer::Now(Value::list(
          page.data.iter().map(Flashcard::to_value).collect(),
        )))
      }
      _ => Err(DataError::UnknownSource(source.into())),
    }
  }

  /// Must equal app.json's `app.id`: a live-replaced module reports this id,
  /// and the binary refuses one whose id differs from its own.
  #[allow(clippy::unnecessary_literal_bound)]
  fn app_id(&self) -> &str {
    "com.assanee.flashcard"
  }

  /// What the app may reach; the module and binary must agree.
  #[allow(clippy::unnecessary_literal_bound)]
  fn grants(&self) -> &str {
    "net.fetch http://localhost:3000"
  }
}
