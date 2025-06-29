use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub mod utils;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FromClient {
  Join {group_name: Arc<String>},
  Post {group_name: Arc<String>, message: Arc<String>},
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FromServer {
  Message {
    group_name: Arc<String>,
    message: Arc<String>
  },
  Error(String),
}


#[test]
fn test_from_client_json() {
  use std::sync::Arc;
  let from_client = FromClient::Post {
    group_name: Arc::new("Dogs".to_string()),
    message: Arc::new("Woof woof".to_string()),
  };

  let json = serde_json::to_string(&from_client).unwrap();
  assert_eq!(json, r#"{"Post":{"group_name":"Dogs","message":"Woof woof"}}"#);

  let from_server = FromServer::Message {
    group_name: Arc::new("Dogs".to_string()),
    message: Arc::new("Woof woof".to_string()),
  };

  let json = serde_json::to_string(&from_server).unwrap();
  assert_eq!(json, r#"{"Message":{"group_name":"Dogs","message":"Woof woof"}}"#);
}
