use crate::prelude::*;

macro_rules! decl_conversion_tests_for {
    (
        $(
            #[doc = $expr: expr]
        )*
        $type:ident
    ) => {
        paste! {
            #[cfg(test)]
            mod [<$type:snake _conversion_tests>] {
                use super::*;

                #[test]
                fn test_conversion() {
                    let internal = [<Internal $type>]::sample();
                    let value = $type::from(internal.clone());
                    let roundtrip_converted: [<Internal $type>] = value.into_internal();
                    assert_eq!(roundtrip_converted, internal);
                }
            }
        }
    }
}

pub(crate) use decl_conversion_tests_for;
