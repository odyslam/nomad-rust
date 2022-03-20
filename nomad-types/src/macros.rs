#[macro_export]
/// Implement deser_nomad_number for a uint type
macro_rules! impl_deser_nomad_number {
    ($($u:ident),*) => {
        $(paste::paste! {
            #[doc = "Permissive deserialization of numbers. Allows numbers, hex strings, and decimal strings"]
            pub fn [<deser_nomad_number_ $u>]<'de, D>(deserializer: D) -> Result<$u, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct NumberOrNumberStringVisitor;

                impl<'de> serde::de::Visitor<'de> for NumberOrNumberStringVisitor {
                    type Value = $u;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter
                            .write_str("an integer, a decimal string, or a 0x-prepended hexadecimal string")
                    }

                    fn [<visit_ $u>]<E>(self, v: $u) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(v)
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        if let Ok(res) = v.parse() {
                            return Ok(res);
                        }

                        if let Some(stripped) = v.strip_prefix("0x") {
                            if stripped.is_empty() {
                                return Ok(0);
                            }
                            if let Ok(res) = $u::from_str_radix(stripped, 16) {
                                return Ok(res);
                            }
                        }

                        Err(E::invalid_value(serde::de::Unexpected::Str(v), &self))
                    }
                }

                deserializer.deserialize_any(NumberOrNumberStringVisitor)
            }
        })*
    };
}
