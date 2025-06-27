use std::vec;

use surf::Error;

async fn many_requsets(urls: &[String]) -> Vec<Result<String, Error>> {
  let client = surf::Client::new();

  let mut handles = Vec::new();
  for url in urls {
    let request = client.get(&url).recv_string();
    handles.push(async_std::task::spawn(request));
  }

  let mut results = vec![];

  for handle in handles {
    results.push(handle.await);
  }

  results
}

fn main() {
  let requests = &["https://example.com".to_string(),
    "https://www.red-bean.com".to_string(),
    "https://www.rust-lang.org".to_string(),
  ];

  let results = async_std::task::block_on(many_requsets(requests));

  for result in results {
    match result {
      Ok(response) => println!("*** {}\n", response),
      Err(error) => println!("*** Error: {}\n", error),
    }
  }
}
