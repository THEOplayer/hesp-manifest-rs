use crate::Result;

pub trait Validate {
    fn validate(&self) -> Result<()>;
}

#[macro_export]
/// will call Self::validate when deserializing with serde (requires `#[serde(remote = "Self")]`)
macro_rules! validate_on_deserialize {
    ( $type:ident ) => {
        impl<'de> Deserialize<'de> for $type {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let object = Self::deserialize(deserializer)?;
                object.validate().map_err(serde::de::Error::custom)?;
                Ok(object)
            }
        }

        impl Serialize for $type {
            fn serialize<S: serde::Serializer>(
                &self,
                serializer: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                Self::serialize(self, serializer)
            }
        }
    };
}
