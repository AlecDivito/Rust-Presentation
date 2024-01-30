use std::{fmt::Display, str::FromStr};

struct Version {
    major: Option<i32>,
    minor: Option<i32>,
    patch: Option<i32>,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.major, self.minor, self.patch) {
            (Some(major), Some(minor), Some(patch)) => write!(f, "{}.{}.{}", major, minor, patch),
            (Some(major), Some(minor), None) => write!(f, "{}.{}", major, minor),
            (Some(major), None, None) => write!(f, "{}", major),
            (None, _, _) | (Some(_), None, _) => unreachable!(),
        }
    }
}

impl FromStr for Version {
    type Err = <i32 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut array = s
            .split('.')
            .map(<i32 as FromStr>::from_str)
            .collect::<Result<Vec<i32>, _>>()?;
        Ok(Version {
            major: array.pop(),
            minor: array.pop(),
            patch: array.pop(),
        })
    }
}

fn test(str: &str) {
    match Version::from_str(str) {
        Ok(version) => println!("SUCCESS: {}", version),
        Err(err) => println!("ERROR: {} not a version: {:?}", str, err),
    }
}

pub fn main() {
    test("hello");
    test("1");
    test("1.10");
    test("1.20.5");
    test("1.aahh.5");
    test("")
}
