use std::{
  collections::HashMap,
  env::current_dir,
  fs::File,
  io::{BufRead, BufReader},
  str::FromStr,
};

pub struct Environment {
  store: HashMap<String, String>,
}

impl Environment {
  pub fn from_file(file_path: &str) -> Self {
    let Ok(file) = File::open(file_path) else {
      panic!("Failed to find environment file at {}; does it exist?", file_path);
    };
    let mut env = Self {
      store: HashMap::<String, String>::new(),
    };
    for line in BufReader::new(file).lines() {
      let Ok(env_line) = line else {
        continue
      };
      let mut found_char = false;
      for char in env_line.chars() {
        match char {
          ' ' => continue,
          '#' => break,
          _ => {
            found_char = true;
            break;
          }
        }
      }
      if !found_char {
        continue;
      };
      let split: Vec<&str> =
        env_line.split('=').map(|chunk| chunk.trim()).collect();
      if split.len() != 2 {
        continue;
      };
      env.store.insert(split[0].to_string(), split[1].to_string());
    }
    env
  }

  pub fn new() -> Self {
    Self::default()
  }

  pub fn get<T>(&self, key: &str) -> Option<T>
  where
    T: FromStr,
  {
    let Some(env_string) = self.store.get(key) else {
      return None;
    };
    if let Ok(parsed) = env_string.parse() {
      Some(parsed)
    } else {
      None
    }
  }

  pub fn require<T>(&self, key: &str) -> T
  where
    T: FromStr,
  {
    let Some(env_string) = self.store.get(key) else {
      panic!("No environment variable was found with key {}", key);
    };
    if let Ok(parsed) = env_string.parse() {
      parsed
    } else {
      panic!("Failed to parse {} as specified type", key)
    }
  }
}

impl Default for Environment {
  fn default() -> Self {
    let Ok(location) = current_dir() else {
      panic!("Failed to read current directory; do you have the right permissions?");
    };
    let file_path = format!("{}/.env", location.display());
    Self::from_file(&file_path)
  }
}
