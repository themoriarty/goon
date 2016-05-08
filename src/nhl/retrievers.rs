extern crate hyper;
extern crate threadpool;
extern crate eventual;


use std::io;
use std::io::{Error, ErrorKind};
use std::io::Read;
use std::sync::Arc;

use self::eventual::Future;
use self::hyper::Client;
//use self::hyper::header::Connection;
use self::threadpool::ThreadPool;

pub use self::eventual::Async;

// simple interface to retrieve data by key
// could be backed by, say, simple http client, or local DB, or a mix
pub trait DataRetriever {
  fn fetch(&self, key: String) -> Future<String, Error>;
}

pub struct SimpleHttpRetriever {
  client: Arc<Client>,
  pool: ThreadPool,
}

impl SimpleHttpRetriever {
  pub fn new(num_threads: usize) -> SimpleHttpRetriever {
    let client = Arc::new(Client::new());
    let pool = ThreadPool::new(num_threads);
    return SimpleHttpRetriever { client: client, pool: pool };
  }
}

impl DataRetriever for SimpleHttpRetriever {
  fn fetch(&self, key: String) -> Future<String, io::Error> {
    let (completer, future) = Future::<String, io::Error>::pair();

    let client_copy = self.client.clone();
    self.pool.execute(move || {
      match client_copy.get(&key).send() {
        Ok(mut response) => {
          let mut body = String::new();
          match response.read_to_string(&mut body) {
            Ok(_) => completer.complete(body),
            Err(e) => completer.fail(e),
          }
        }
        Err(e) => completer.fail(Error::new(ErrorKind::Other, e))
      }
    });
    return future;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn simple_download() {
    let retriever = SimpleHttpRetriever::new(1);
    let robots_txt = retriever.fetch("http://google.com/robots.txt".to_string()).await().unwrap();
    assert!(robots_txt.len() > 0);
  }

  #[test]
  fn parallel_download() {
    let retriever = SimpleHttpRetriever::new(1);
    // using external echo service
    let f1 = retriever.fetch("http://scooterlabs.com/echo?XXX".to_string())
      .and_then(|v| {assert!(v.as_str().contains("XXX")); Ok(v)});
    let f2 = retriever.fetch("http://scooterlabs.com/echo?YYY".to_string())
      .and_then(|v| {assert!(v.as_str().contains("YYY")); Ok(v)});
    f1.and(f2).await().unwrap();
  }

}