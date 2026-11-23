//! Writes `OUT_DIR/api.rs`: the flashcards server's origin (`API`) and the
//! grant that admits it (`GRANTS`), fixed at build.
//!
//! `API_BASE_URL` comes from the environment, then `.env.local`, then `.env`
//! beside app.json (Exact's order for `app.ts` settings), else localhost.
//! It ships in the app, so it is never a secret.

use std::path::{Path, PathBuf};

const DEFAULT: &str = "http://localhost:3000";

/// `API_BASE_URL=…` from a dotenv file, quotes trimmed.
fn from_file(path: &Path) -> Option<String> {
  std::fs::read_to_string(path)
    .ok()?
    .lines()
    .find_map(|line| {
      let value = line.trim().strip_prefix("API_BASE_URL=")?.trim();
      Some(value.trim_matches(|c| c == '"' || c == '\'').to_string())
    })
}

fn main() {
  let app = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("..");
  let files = [app.join(".env.local"), app.join(".env")];
  println!("cargo:rerun-if-env-changed=API_BASE_URL");
  for file in &files {
    println!("cargo:rerun-if-changed={}", file.display());
  }
  let api = std::env::var("API_BASE_URL")
    .ok()
    .or_else(|| files.iter().find_map(|file| from_file(file)))
    .filter(|api| !api.is_empty())
    .unwrap_or_else(|| DEFAULT.into());
  let api = api.trim_end_matches('/');
  let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("api.rs");
  std::fs::write(
    out,
    format!(
      "/// The flashcards server's origin (`API_BASE_URL`).\nconst API: &str = {api:?};\n\
       /// What the app may reach: the server above.\nconst GRANTS: &str = {:?};\n",
      format!("net.fetch {api}")
    ),
  )
  .unwrap();
}
