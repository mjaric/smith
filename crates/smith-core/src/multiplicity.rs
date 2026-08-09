//! Multiplicity of association ends and attributes (`REQ-MM-014`).
//!
//! Notation: `lower..upper`, with the abbreviations `1` (`1..1`) and `*`
//! (`0..*`). `upper < lower` is invalid and rejected on construction and parse.

use std::fmt;
use std::str::FromStr;

/// Inclusive lower bound and optional inclusive upper bound of a multiplicity.
///
/// `upper` of [`None`] means unbounded (`*`). Two multiplicities are equal only
/// when both bounds match exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Multiplicity {
    /// Inclusive lower bound.
    pub lower: u32,
    /// Inclusive upper bound, or [`None`] for unbounded (`*`).
    pub upper: Option<u32>,
}

/// Error raised when a multiplicity is malformed or its bounds are inconsistent.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MultiplicityError {
    /// The upper bound is below the lower bound (`upper < lower`).
    #[error("multiplicity upper bound {upper} is below lower bound {lower}")]
    UpperBelowLower {
        /// The (too-large) lower bound.
        lower: u32,
        /// The (too-small) upper bound that was rejected.
        upper: u32,
    },
    /// A multiplicity string could not be parsed.
    #[error("invalid multiplicity {input:?} (expected `1`, `*`, `lower..upper`, or `lower..*`)")]
    InvalidFormat {
        /// The text that failed to parse.
        input: String,
    },
}

impl Multiplicity {
    /// Build a multiplicity, rejecting `upper < lower`.
    ///
    /// # Errors
    /// Returns [`MultiplicityError::UpperBelowLower`] when `upper` is `Some`
    /// and strictly less than `lower`.
    pub fn new(lower: u32, upper: Option<u32>) -> Result<Self, MultiplicityError> {
        if let Some(upper_value) = upper {
            if upper_value < lower {
                return Err(MultiplicityError::UpperBelowLower {
                    lower,
                    upper: upper_value,
                });
            }
        }
        Ok(Self { lower, upper })
    }

    /// Whether this multiplicity is unbounded (`upper` is [`None`]).
    #[must_use]
    pub const fn is_unbounded(self) -> bool {
        self.upper.is_none()
    }
}

impl fmt::Display for Multiplicity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self.lower, self.upper) {
            // `1` and `*` abbreviations (spec-defined).
            (lower, Some(upper)) if lower == upper => write!(f, "{lower}"),
            (0, None) => write!(f, "*"),
            (lower, None) => write!(f, "{lower}..*"),
            (lower, Some(upper)) => write!(f, "{lower}..{upper}"),
        }
    }
}

impl FromStr for Multiplicity {
    type Err = MultiplicityError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let trimmed = text.trim();
        if trimmed.is_empty() {
            return Err(invalid(text));
        }
        if trimmed == "*" {
            return Ok(Self {
                lower: 0,
                upper: None,
            });
        }
        let Some((lower_text, upper_text)) = trimmed.split_once("..") else {
            let lower = trimmed.parse::<u32>().map_err(|_| invalid(text))?;
            return Self::new(lower, Some(lower)).map_err(|_| invalid(text));
        };
        let lower = lower_text
            .trim()
            .parse::<u32>()
            .map_err(|_| invalid(text))?;
        let upper = if upper_text.trim() == "*" {
            None
        } else {
            Some(
                upper_text
                    .trim()
                    .parse::<u32>()
                    .map_err(|_| invalid(text))?,
            )
        };
        Self::new(lower, upper).map_err(|_| invalid(text))
    }
}

/// Builds an [`MultiplicityError::InvalidFormat`] preserving the original text.
fn invalid(text: &str) -> MultiplicityError {
    MultiplicityError::InvalidFormat {
        input: text.to_string(),
    }
}
