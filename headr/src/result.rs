use crate::PRG_NAME;
use anyhow;

pub struct HeadrResult {
  outputs: Vec<HeadrFileCount>,
}

impl HeadrResult {
  pub fn new() -> Self {
      Self { outputs: vec![] }
  }

  pub fn add_error(&mut self, filename: &str, err: anyhow::Error) {
      self.outputs.push(HeadrFileCount::Err(format!("{PRG_NAME}: {filename}: {}\n", err)));
  }

  pub fn add_newline(&mut self) {
      self.outputs.push(HeadrFileCount::Output("\n".to_string()));
  }

  pub fn add_outputs(&mut self, outputs: &Vec<String>) {
      for output in outputs {
          self.outputs
              .push(HeadrFileCount::Output(output.to_string()));
      }
  }

  pub fn print_results(&self) {
      for item in &self.outputs {
          item.print();
      }
  }
}


enum HeadrFileCount {
  Err(String),
  Output(String),
}

impl HeadrFileCount {
  fn print(&self) {
      match self {
          Self::Err(msg) => eprint!("{msg}"),
          Self::Output(msg) => print!("{msg}"),
      }
  }
}