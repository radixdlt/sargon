use crate::prelude::*;

decl_identified_vec_of!(
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An ordered set of [`SecurityQuestion`]s user has selected to use
    /// in their `SecurityQuestionsFactorSource` - a factor they typically
    /// use for the `ConfirmationRole`.
    /// ❗️ NOT PRODUCTION READY YET ❗️
    Security_NOT_PRODUCTION_READY_Questions,
    Security_NOT_PRODUCTION_READY_Question
);

// impl HasSampleValues for Security_NOT_PRODUCTION_READY_Questions {
//     fn sample() -> Self {
//         type Q = Security_NOT_PRODUCTION_READY_Question;
//         Self::from_iter([
//             Q::q00(),
//             Q::q01(),
//             Q::q02(),
//             Q::q03(),
//             Q::q04(),
//             Q::q05(),
//         ])
//     }
//     fn sample_other() -> Self {
//         type Q = Security_NOT_PRODUCTION_READY_Question;
//         Self::from_iter([
//             Q::q06(),
//             Q::q07(),
//             Q::q08(),
//             Q::q09(),
//             Q::q10(),
//             Q::q11(),
//         ])
//     }
// }
