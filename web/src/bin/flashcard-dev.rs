//! The resident dev driver for Flashcards: `flashcard-dev <source> <out>`.
//! Run through `bun exact.mjs web`, which pushes each plan to the page.

fn main() -> std::process::ExitCode {
  exact_web::dev::main::<flashcard_data::FlashcardData>()
}
