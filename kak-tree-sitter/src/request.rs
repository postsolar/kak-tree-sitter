//! Requests that can be sent to the server from Kakoune.

use std::fmt::Debug;

use serde::{Deserialize, Serialize};

/// Unidentified request (i.e. not linked to a given session).
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum UnixRequest {
  /// Inform KTS that a new session exists and that we should be sending back the Kakoune commands to get KTS features.
  NewSession { name: String, client: String },

  /// Inform KTS that a session has exited.
  SessionExit { name: String },

  /// Ask KTS to reload its configuration and reload grammars / queries.
  Reload,

  /// Ask KTS to shutdown.
  Shutdown,
}

impl UnixRequest {
  /// Add a session name to a [`UnidentifiedRequest`], replacing it if one was already provided.
  pub fn with_session(self, name: impl Into<String>) -> Self {
    let name = name.into();

    match self {
      UnixRequest::NewSession { client, .. } => UnixRequest::NewSession { name, client },

      UnixRequest::SessionExit { .. } => UnixRequest::SessionExit { name },

      _ => self,
    }
  }
}

/// Request payload.
///
/// Request payload are parameterized with the « origin » at which requests are expected.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Request {
  /// Try enabling highlighting for a given filetype.
  ///
  /// This request starts a “highlighting session.” The response will not replay with « supports highlighting » or
  /// « does not support highlighting », but instead will insert the Kakoune commands to ask for highlights only if the
  /// filetype is supported.
  TryEnableHighlight { lang: String, client: String },

  /// Ask to highlight the given buffer.
  ///
  /// The content of the buffer is streamed right after in the same command FIFO file the request was sent in.
  Highlight {
    client: String,
    buffer: String,
    lang: String,
    timestamp: u64,
  },
}

impl Request {
  pub fn client_name(&self) -> Option<&str> {
    match self {
      Request::TryEnableHighlight { client, .. } => Some(client.as_str()),
      Request::Highlight { client, .. } => Some(client.as_str()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Request;

  #[test]
  fn serialization() {
    let req = Request::Highlight {
      client: "client0".to_owned(),
      buffer: "/tmp/a.rs".to_owned(),
      lang: "rust".to_owned(),
      timestamp: 0,
    };
    let expected =
      r#"{"type":"highlight","client":"client0","buffer":"/tmp/a.rs","lang":"rust","timestamp":0}"#;
    let serialized = serde_json::to_string(&req);

    assert_eq!(serialized.unwrap(), expected);
  }
}
