extern crate rustc_serialize;
extern crate goon;

use goon::nhl;
use goon::nhl::*;

use rustc_serialize::json;

fn main() {
  let goon = nhl::SimpleHttpRetriever::new(10);

  goon
  .fetch("https://statsapi.web.nhl.com/api/v1/game/2015030244/feed/live".to_string())
  .and_then(|v| {
    let decoded: nhl::LiveFeed = json::decode(&v).unwrap();

    println!("data: {:?}", decoded);
    Ok(v)
  }).await().unwrap();
  //
  //  let object = TestStruct {
  //    data_int: 1,
  //    data_str: "homura".to_string(),
  //    data_vector: vec![2,3,4,5],
  //  };
  //
  //  // Serialize using `json::encode`
  //  let encoded = json::encode(&object).unwrap();
  //
  //  // Deserialize using `json::decode`
  //  let decoded: TestStruct = json::decode(&encoded).unwrap();
  //
  //  println!("data: {}", encoded);
}

