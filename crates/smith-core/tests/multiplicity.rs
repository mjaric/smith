//! `REQ-MM-014`: multiplicity parses, formats, and rejects `upper < lower`.

use std::str::FromStr;

use smith_core::{Multiplicity, MultiplicityError};

#[test]
fn multiplicity_parses_formats_and_rejects_upper_below_lower() {
    // Abbreviations parse to their bound structs and format back canonically.
    let one = Multiplicity {
        lower: 1,
        upper: Some(1),
    };
    assert_eq!(Multiplicity::from_str("1"), Ok(one));
    assert_eq!(one.to_string(), "1");

    let star = Multiplicity {
        lower: 0,
        upper: None,
    };
    assert_eq!(Multiplicity::from_str("*"), Ok(star));
    assert_eq!(star.to_string(), "*");

    let zero_one = Multiplicity {
        lower: 0,
        upper: Some(1),
    };
    assert_eq!(Multiplicity::from_str("0..1"), Ok(zero_one));
    assert_eq!(zero_one.to_string(), "0..1");

    let one_five = Multiplicity {
        lower: 1,
        upper: Some(5),
    };
    assert_eq!(Multiplicity::from_str("1..5"), Ok(one_five));
    assert_eq!(one_five.to_string(), "1..5");

    // Full forms normalize to the canonical abbreviations.
    assert_eq!(
        Multiplicity::from_str("1..1"),
        Ok(Multiplicity {
            lower: 1,
            upper: Some(1),
        })
    );
    assert_eq!(
        Multiplicity::from_str("0..*"),
        Ok(Multiplicity {
            lower: 0,
            upper: None
        })
    );
    assert_eq!(
        Multiplicity::from_str("3..*"),
        Ok(Multiplicity {
            lower: 3,
            upper: None,
        })
    );
    assert_eq!(
        Multiplicity {
            lower: 1,
            upper: Some(1)
        }
        .to_string(),
        "1"
    );
    assert_eq!(
        Multiplicity {
            lower: 0,
            upper: None
        }
        .to_string(),
        "*"
    );
    assert_eq!(
        Multiplicity {
            lower: 3,
            upper: None,
        }
        .to_string(),
        "3..*"
    );

    // `upper < lower` is rejected, both on construction and on parse.
    assert_eq!(
        Multiplicity::new(3, Some(2)),
        Err(MultiplicityError::UpperBelowLower { lower: 3, upper: 2 })
    );
    assert!(Multiplicity::from_str("3..2").is_err());
}
