use clap::{command, Parser};

const ONE_KB: u32 = 1000;
const ONE_KIB: u32 = 1024;
const NUM_BYTE: u32 = 512;

#[derive(Parser, Debug)]
#[command(version,about,long_about=None)]
/// Rust implementation of head
pub struct Headr {
    /// list of files to concatenate
    #[arg(default_value("-"))]
    pub files: Vec<String>,
    /// Print the first K bytes of each file; with a leading -,
    /// print all but the last K bytes of each file
    #[arg(short('c'), long("bytes"), conflicts_with("lines"), allow_hyphen_values(true), value_parser = parse_arg)]
    pub bytes: Option<i64>,
    /// Print the first K lines of each file instead of the first 10;
    /// with a leading -, print all but the last K lines of each file
    #[arg(short('n'), long("lines"), default_value("10"), allow_hyphen_values(true), value_parser = parse_arg)]
    pub lines: i64,
}

impl Headr {
  pub fn read_bytes(&self) -> bool {
        self.bytes.is_some()
    }
}

// Parse number of lines or bytes to read command line argument
fn parse_arg(s: &str) -> Result<i64, String> {
  let mut num = String::new();
  let mut unit = String::new();
  let mut negative = false;

  for (i, c) in s.chars().enumerate() {
      if i == 0 && c == '-' {
          negative = true
      } else if c.is_digit(10) {
          num.push(c);
      } else {
          unit.push(c);
      }
  }
  let num = num.parse::<i64>().map_err(|_| "must be a numeric string")?;

  if num == 0 {
      return Err("must be non-zero".to_string());
  }

  let unit = match unit.as_str() {
      "K" => ONE_KIB,
      "M" => ONE_KIB * ONE_KIB,
      "G" => ONE_KIB * ONE_KIB * ONE_KIB,
      "b" => NUM_BYTE,
      "kB" => ONE_KB,
      "MB" => ONE_KB * ONE_KB,
      "GB" => ONE_KB * ONE_KB * ONE_KB,
      "" => 1,
      _ => {
          return Err("unknown unit".to_string());
      }
  };

  match num.checked_mul(unit.into()) {
      None => Err("value is too large".to_string()),
      Some(val) => {
          let res = if negative { -1 * val } else { val };
          Ok(res)
      }
  }
}
