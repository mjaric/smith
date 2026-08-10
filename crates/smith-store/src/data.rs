//! Validated JSON `data` blob round-trip helpers (REQ-PERS-005).
//!
//! The `data` / `tag_values` / `canvas_state` TEXT columns hold kind-specific
//! JSON blobs. Blobs are validated by serde on both write and read; raw
//! database content is never trusted. The typed structs per element kind are
//! owned by `smith-model` — this module only guarantees the JSON envelope.

/// A data blob failed to serialize, or did not deserialize into the expected
/// shape.
#[derive(Debug, thiserror::Error)]
pub enum DataError {
    /// Serialization into JSON failed.
    #[error("data blob serialization failed: {detail}")]
    Serialize {
        /// The `serde_json` error.
        #[source]
        detail: serde_json::Error,
    },
    /// The stored blob was not valid JSON of the expected shape.
    #[error("data blob is not valid JSON of the expected shape: {detail}")]
    Deserialize {
        /// The `serde_json` error.
        #[source]
        detail: serde_json::Error,
    },
}

/// Serialize a kind-specific data struct into a validated JSON blob.
///
/// The result is guaranteed valid JSON (`serde_json` produced it), so it may be
/// written directly into a `data` column: blobs are validated on write, never
/// trusted raw (REQ-PERS-005).
///
/// # Errors
///
/// Returns [`DataError::Serialize`] if `value` cannot be represented as JSON.
pub fn serialize_data<T: serde::Serialize>(value: &T) -> Result<String, DataError> {
    serde_json::to_string(value).map_err(|detail| DataError::Serialize { detail })
}

/// Deserialize a stored `data` blob into a kind-specific struct.
///
/// The blob is never trusted raw: anything that is not valid JSON of the
/// expected shape is rejected.
///
/// # Errors
///
/// Returns [`DataError::Deserialize`] if the blob is invalid JSON or has the
/// wrong shape for `T`.
pub fn deserialize_data<T: serde::de::DeserializeOwned>(blob: &str) -> Result<T, DataError> {
    serde_json::from_str(blob).map_err(|detail| DataError::Deserialize { detail })
}
