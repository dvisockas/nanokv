use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
  let mut db = Database::new().expect("DB init failed");
  let mut arguments = std::env::args().skip(1);
  let key  = match arguments.next() {
    Some(c) => { c },
    None => { return println!("No command supplied") }
  };

  let val = match arguments.next() {
    Some(c) => { c },
    None => { "".to_owned() }
  };

  if val.len() > 0 {
    db.insert(key, val);
  } else {
    db.get(key);
  }
}

struct Database {
  db_path: String,
  map: HashMap<String, String>,
}

impl Database {
  fn new() -> Result<Database, std::io::Error> {
    let db_path = "kv.db";

    if !std::path::Path::new(db_path).exists() {
      std::fs::File::create(db_path)?;
    }

    let contents = std::fs::read_to_string(db_path)?;

    let mut map = HashMap::new();

    for line in contents.lines() {
      let mut chunks = line.splitn(2, '\t');
      let key = chunks.next().expect("No Key");
      let val = chunks.next().expect("No Val");
      map.insert(key.to_owned(), val.to_owned());
    };

    Ok(Database { map: map, db_path: db_path.to_owned() })
  }

  fn insert(&mut self, key:String, val:String) {
    self.map.insert(key.to_owned(), val.to_owned());

    let mut file = OpenOptions::new().append(true).open(&self.db_path).unwrap();
    let content = format!("{}\t{}\n", key, val);
    file.write_all(content.as_bytes()).unwrap();
    println!("OK");
  }

  fn get(&mut self, key:String) {
    match self.map.get(&key) {
      Some(val) => { println!("{}", val) },
      None => println!("No key named '{}'", key),
    };
  }
}
