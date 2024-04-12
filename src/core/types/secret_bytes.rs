
use crate::prelude::*;

#[macro_export]
macro_rules! secret_bytes {
    (
        $(
            #[doc = $expr: expr]
        )*
        $struct_name: ident, 
        $byte_count: literal
    ) => {
        paste! {

            $(
                #[doc = $expr]
            )*
            #[derive(Zeroize, ZeroizeOnDrop, derive_more::Debug, derive_more::Display)]
            #[debug("OBFUSCATED")]
            #[display("OBFUSCATED")]
            pub struct $struct_name(Box<[u8; Self::LENGTH]>);
           
            impl $struct_name {
                pub const LENGTH: usize = $byte_count;
            
                pub(crate) fn new(bytes: [u8; Self::LENGTH]) -> Self {
                    Self(Box::new(bytes))
                }
         
                #[allow(unused)]
                pub(crate) fn is_zeroized(&self) -> bool {
                    *self.0 == [0; Self::LENGTH]
                }
            }
            
            #[cfg(test)]
            mod [< $struct_name:snake tests >] {
                use super::*;
                use std::mem;
                use std::ops::Range;
            
                #[allow(clippy::upper_case_acronyms)]
                type SUT = $struct_name;
            
                #[test]
                fn zeroize() {
                    let mut sut = SUT::new([0xff; SUT::LENGTH]);
                    assert!(!sut.is_zeroized());
                    sut.zeroize();
                    assert!(sut.is_zeroized());
                }

                #[test]
                fn debug_obfuscates_secret() {
                    let sut = SUT::new([0xff; SUT::LENGTH]);
                    assert_eq!(format!("{:?}", sut), "OBFUSCATED");
                }

                #[test]
                fn display_obfuscates_secret() {
                    let sut = SUT::new([0xff; SUT::LENGTH]);
                    assert_eq!(format!("{}", sut), "OBFUSCATED");
                }
            
                #[test]
                fn zeroize_on_drop() {
                    let sut = SUT::new([0xff; SUT::LENGTH]);
                   
                    let view = sut.0.as_ptr();
                    let range = Range { start: 0, end: SUT::LENGTH as isize };
                 
                    for i in range.clone() {
                        assert_eq!(unsafe { *view.offset(i) }, 0xff);
                    }
            
                    drop(sut);
            
                    for i in range.clone() {
                        assert_eq!(unsafe { *view.offset(i) }, 0x00);
                    }
            
                }
            }
            
        }
    }
}
