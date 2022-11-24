#[macro_export]
macro_rules! generate_mapping {
    ($entity:ty, $model:ty, $field_count:expr) => {
        seq_macro::seq!(N in 1..$field_count {

            impl Into<$entity> for $model {
                fn into(self) -> $entity {
                    let (v0, #(v~N,)*) = self.into();

                    $entity::from((v0.into(), #(v~N.into(),)*))
                }
            }

            impl From<$entity> for $model {
                fn from(value: $entity) -> Self {
                    let (v0, #(v~N,)*) = value.into();

                    Self::from((v0.into(), #(v~N.into(),)*))
                }
            }
        });
    };
}