// macro_rules! decl_ref {
//     (
//         $inner: ty
//     ) => {
//         paste! {

//             #[derive(Debug, uniffi::Object)]
//             #[uniffi::export(Debug, Eq, Hash)]
//             pub struct [< Ref $inner:camel >] {
//                 pub profile: RwLock<Option<Profile>>,
//             }

//             impl RefProfile {
//                 fn with_profile(profile: Profile) -> Self {
//                     Self {
//                         profile: RwLock::new(Some(profile)),
//                     }
//                 }
//             }

//             impl std::hash::Hash for RefProfile {
//                 fn hash<H>(&self, state: &mut H)
//                 where
//                     H: std::hash::Hasher,
//                 {
//                     match self.profile.read() {
//                         Ok(ref guard) => {
//                             state.write_u8(1);
//                             match guard.as_ref() {
//                                 Some(prof) => {
//                                     prof.hash(state);
//                                     state.write_u8(100);
//                                 }
//                                 None => state.write_u8(200),
//                             }
//                         }
//                         _ => {
//                             state.write_u8(255);
//                         }
//                     }
//                 }
//             }

//             impl Eq for RefProfile {}
//             impl PartialEq for RefProfile {
//                 fn eq(&self, other: &Self) -> bool {
//                     {
//                         match self.profile.read() {
//                             Ok(ref rhs) => match other.profile.read() {
//                                 Ok(ref lhs) => match rhs.as_ref() {
//                                     Some(r) => match lhs.as_ref() {
//                                         Some(l) => r == l,
//                                         None => false,
//                                     },
//                                     None => lhs.as_ref().is_none(),
//                                 },
//                                 _ => false,
//                             },
//                             _ => false,
//                         }
//                     }
//                 }
//             }

//             impl From<Profile> for RefProfile {
//                 fn from(value: Profile) -> Self {
//                     Self::with_profile(value)
//                 }
//             }

//         }
//     }
// }
