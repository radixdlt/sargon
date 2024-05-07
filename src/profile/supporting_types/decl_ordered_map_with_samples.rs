use crate::prelude::*;

macro_rules! decl_ordered_map {
    (
        $(
            #[doc = $expr: expr]
        )*
        $collection_type: ident,
        $element_type: ident
    ) => {
        paste! {
            $(
                #[doc = $expr]
            )*
			pub type $collection_type = OrderedMap<$element_type>;

			#[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample >]() -> $collection_type {
                $collection_type::sample()
            }

            #[uniffi::export]
            pub fn [< new_ $collection_type:snake _sample_other >]() -> $collection_type {
                $collection_type::sample_other()
            }
		}
	};

	// (
    //     $(
    //         #[doc = $expr: expr]
    //     )*
    //     $collection_type: ident,
    //     $element_type: ident
    // ) => {
	// 	paste! {
	// 		decl_ordered_map!(
	// 			$(
    //                 #[doc = $expr]
    //             )*
	// 			[< $element_type s>],
	// 			$element_type,
    //             true
	// 		);
	// 	}
	// };

    (
        $(
            #[doc = $expr: expr]
        )*
        $element_type: ident
    ) => {
		paste! {
			decl_ordered_map!(
				$(
                    #[doc = $expr]
                )*
				[< $element_type s>],
				$element_type
			);
		}
	};
}

// macro_rules! decl_never_empty_ordered_map {

// 	(
//         $(
//             #[doc = $expr: expr]
//         )*
//         $collection_type: ident,
//         $element_type: ident
//     ) => {
// 		paste! {
// 			decl_ordered_map!(
// 				$(
//                     #[doc = $expr]
//                 )*
// 				[< $element_type s>],
// 				$element_type,
//                 false
// 			);
// 		}
// 	};
//     (
//         $(
//             #[doc = $expr: expr]
//         )*
//         $element_type: ident
//     ) => {
// 		paste! {
// 			decl_ordered_map!(
// 				$(
//                     #[doc = $expr]
//                 )*
// 				[< $element_type s>],
// 				$element_type,
//                 false
// 			);
// 		}
// 	};
// }

pub(crate) use decl_ordered_map;
