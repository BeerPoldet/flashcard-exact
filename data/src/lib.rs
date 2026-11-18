//! Flashcards' data source: answers the sources `app.contract` declares,
//! in place of `app.ts`.

#![deny(missing_docs)]

use exact_plan::Value;
use exact_runner::{DataError, DataSource};

/// The app's data source.
#[derive(Default, Clone, Copy)]
pub struct FlashcardData;

/// `greeting(name)`: a `shape Greeting` (fields in declared order: `text`).
fn greeting(args: &[Value]) -> Result<Value, DataError> {
  let name = args
    .first()
    .and_then(Value::as_str)
    .ok_or_else(|| DataError::BadArguments("greeting(name: string)".into()))?;
  Ok(Value::record(vec![Value::str(&format!(
    "Hey from {name}."
  ))]))
}

impl DataSource for FlashcardData {
  fn query(&mut self, source: &str, args: &[Value]) -> Result<Value, DataError> {
    match source {
      "greeting" => greeting(args),
      _ => Err(DataError::UnknownSource(source.into())),
    }
  }

  /// Must equal app.json's `app.id`: a live-replaced module reports this id,
  /// and the binary refuses one whose id differs from its own.
  #[allow(clippy::unnecessary_literal_bound)]
  fn app_id(&self) -> &str {
    "com.assanee.flashcard"
  }
  // `grants` keeps its default (none, as `app.ts`'s empty `grants`). Add it
  // with `net.fetch …` once a source fetches; the module and binary must agree.
}
