#[macro_export]
macro_rules! define_constant_string {
    ($struct_name:ident => $value:literal) => {
        #[derive(Debug)]
        pub struct $struct_name;

        #[cfg(feature = "serialize")]
        impl ::serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::ser::Serializer,
            {
                serializer.serialize_str($value)
            }
        }
    };
}
