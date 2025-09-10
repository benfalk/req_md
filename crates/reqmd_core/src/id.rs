use ::uuid::Uuid;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
pub struct Id(Uuid);

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct IdError(pub ::uuid::Error);

impl Id {
    pub fn generate() -> Self {
        Id(Uuid::now_v7())
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s).map(Id).map_err(IdError)
    }
}
