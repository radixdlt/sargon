use crate::prelude::*;

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A specification of expected format for an answer to a security question.
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    /// E.g. `"<CITY>, <YEAR>"`
    pub answer_structure: String,

    /// An example of a possible answer that matches `answer_structure`.
    /// E.g. `"Berlin, 1976"`
    pub example_answer: String,

    /// If user is about to select the question:
    /// `"What was the name of your first stuffed animal?"`
    ///
    /// Then we can discourage the user from selecting that question
    /// if the answer is in `["Teddy", "Peter Rabbit", "Winnie (the Poh)"]`
    pub unsafe_answers: Vec<String>,
}

impl Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    pub fn with_details(
        structure: impl AsRef<str>,
        example: impl AsRef<str>,
        unsafe_answers: impl IntoIterator<Item = &'static str>,
    ) -> Self {
        Self {
            answer_structure: structure.as_ref().to_owned(),
            example_answer: example.as_ref().to_owned(),
            unsafe_answers: unsafe_answers
                .into_iter()
                .map(|x| x.to_owned())
                .collect_vec(),
        }
    }

    pub fn new(structure: impl AsRef<str>, example: impl AsRef<str>) -> Self {
        Self::with_details(structure, example, [])
    }

    pub fn name() -> Self {
        Self::new("<Name>", "Maria")
    }

    pub fn location() -> Self {
        Self::with_details(
            "<Location>",
            "At bus stop outside of Dallas",
            ["Specifying only a country as location would be unsafe"],
        )
    }

    pub fn preset_city_and_year() -> Self {
        Self::new("<CITY>, <YEAR>", "Berlin, 1976")
    }
}

impl HasSampleValues for Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat {
    fn sample() -> Self {
        Self::preset_city_and_year()
    }
    fn sample_other() -> Self {
        Self::name()
    }
}

/// ❗️ NOT PRODUCTION READY YET ❗️
/// A security question
/// ❗️ NOT PRODUCTION READY YET ❗️
#[derive(
    Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, uniffi::Record,
)]
#[serde(rename_all = "camelCase")]
#[allow(non_camel_case_types)]
pub struct Security_NOT_PRODUCTION_READY_Question {
    pub id: u16,     // FIXME: newtype
    pub version: u8, // FIXME: newtype
    pub kind: SecurityQuestionKind,
    pub question: String,
    pub expected_answer_format:
        Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat,
}

impl AsRef<str> for Security_NOT_PRODUCTION_READY_Question {
    fn as_ref(&self) -> &str {
        &self.question
    }
}

impl Identifiable for Security_NOT_PRODUCTION_READY_Question {
    type ID = u16; // FIXME: newtype

    /// Return `Element`'s globally unique and stable ID, used to uniquely identify
    /// the `Element` in the `IdentifiedVecOf` collection of elements.
    fn id(&self) -> Self::ID {
        self.id.clone()
    }
}

impl Security_NOT_PRODUCTION_READY_Question {
    pub fn with_details(
        id: u16,
        version: u8,
        kind: SecurityQuestionKind,
        question: impl AsRef<str>,
        expected_answer_format: Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat,
    ) -> Self {
        Self {
            id,
            version,
            kind,
            question: question.as_ref().to_owned(),
            expected_answer_format,
        }
    }

    fn freeform_with_id(
        id: u16,
        question: impl AsRef<str>,
        expected_answer_format: Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat,
    ) -> Self {
        Self::with_details(
            id,
            1,
            SecurityQuestionKind::Freeform,
            question,
            expected_answer_format,
        )
    }
}

impl Security_NOT_PRODUCTION_READY_Question {
    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///  
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn failed_exam() -> Self {
        Self::freeform_with_id(
            0,
            "What was the first exam you failed",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::new(
                "<SCHOOL>, <SCHOOL_GRADE>, <SUBJECT>",
                "MIT, year 2, Physics",
            ),
        )
    }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q00() -> Self { Self::failed_exam() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///  
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn parents_met() -> Self {
        Self::freeform_with_id(
            1,
            "In which city and which year did your parents meet?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::preset_city_and_year()
        )
    }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q01() -> Self { Self::parents_met() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn first_concert() -> Self {
        Self::freeform_with_id(
            2,
            "What was the first concert you attended?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::new(
                "<ARTIST>, <LOCATION>, <YEAR>",
                "Jean-Michel Jarre, Paris La Défense, 1990",
            ),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q02() -> Self { Self::first_concert() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn first_kiss_whom() -> Self {
        Self::freeform_with_id(
            3,
            "What was the name of the boy or the girl you first kissed?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q03() -> Self { Self::first_kiss_whom() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn first_kiss_location() -> Self {
        Self::freeform_with_id(
            4,
            "Where were you when you had your first kiss?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::location(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q04() -> Self { Self::first_kiss_location() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn spouse_met() -> Self {
        Self::freeform_with_id(
            5,
            "In what city and which year did you meet your spouse/significant other?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::preset_city_and_year(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q05() -> Self { Self::spouse_met() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn child_middle_name() -> Self {
        Self::freeform_with_id(
            6,
            "What is the middle name of your youngest child?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q06() -> Self { Self::child_middle_name() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by NordVPN][link].
    ///
    /// [link]: https://nordvpn.com/blog/security-questions/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn stuffed_animal() -> Self {
        Self::freeform_with_id(
            7,
            "What was the name of your first stuffed animal?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::with_details(
                "<NAME>",
                "Oinky piggy pig",
                ["Teddy", "Cat", "Dog", "Winnie (the Poh)", "(Peter) Rabbit"],
            ),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q07() -> Self { Self::stuffed_animal() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by ExpressVPN][link].
    ///
    /// [link]: https://www.expressvpn.com/blog/how-to-choose-a-security-question/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn oldest_cousin() -> Self {
        Self::freeform_with_id(
            8, 
            "What is your oldest cousin’s middle name?", 
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::with_details("<NAME>", "Maria", ["Don’t use this one if you and your cousin are very close and have plenty of mutual friends."]))
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q08() -> Self { Self::oldest_cousin() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by ExpressVPN][link].
    ///
    /// [link]: https://www.expressvpn.com/blog/how-to-choose-a-security-question/
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn teacher_grade3() -> Self {
        Self::freeform_with_id(
            9,
            "What was the last name of your third grade teacher?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q09() -> Self { Self::teacher_grade3() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by OWASP][link].
    ///
    /// [link]:  https://cheatsheetseries.owasp.org/cheatsheets/Choosing_and_Using_Security_Questions_Cheat_Sheet.html
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn applied_uni_no_attend() -> Self {
        Self::freeform_with_id(
            10,
            "What is the name of a college you applied to but didn’t attend?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::new(
                "<UNIVERSITY NAME>",
                "Oxford",
            ),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q10() -> Self { Self::applied_uni_no_attend() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by OWASP][link].
    ///
    /// [link]:  https://cheatsheetseries.owasp.org/cheatsheets/Choosing_and_Using_Security_Questions_Cheat_Sheet.html
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn first_school() -> Self {
        Self::freeform_with_id(
            11,
            " What was the name of the first school you remember attending?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::new(
                "<SCHOOL NAME>",
                "Hogwartz",
            ),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q11() -> Self { Self::first_school() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by OWASP][link].
    ///
    /// [link]:  https://cheatsheetseries.owasp.org/cheatsheets/Choosing_and_Using_Security_Questions_Cheat_Sheet.html
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn math_teacher_highschool() -> Self {
        Self::freeform_with_id(
            12,
            "What was your maths teacher's surname in your 8th year of school?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q12() -> Self { Self::math_teacher_highschool() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question by OWASP][link].
    ///
    /// [link]:  https://cheatsheetseries.owasp.org/cheatsheets/Choosing_and_Using_Security_Questions_Cheat_Sheet.html
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn drivings_instructor() -> Self {
        Self::freeform_with_id(
            13,
            "What was your driving instructor's first name?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q13() -> Self { Self::drivings_instructor() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question in spreadsheet][sheet], linked to [from].
    ///
    /// [from]: https://goodsecurityquestions.com/examples/
    /// [sheet]: https://docs.google.com/spreadsheets/d/1Mzg60sJYLzUzCJhe-_brprx-KRolvLclcykf4H4hF-c/edit#gid=0
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn street_friend_highschool() -> Self {
        Self::freeform_with_id(
            14,
            "What was the street name where your best friend in high school lived (street name only)?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q14() -> Self { Self::street_friend_highschool() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question in spreadsheet][sheet], linked to [from].
    ///
    /// [from]: https://goodsecurityquestions.com/examples/
    /// [sheet]: https://docs.google.com/spreadsheets/d/1Mzg60sJYLzUzCJhe-_brprx-KRolvLclcykf4H4hF-c/edit#gid=0
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn friend_kindergarten() -> Self {
        Self::freeform_with_id(
            15,
            "What was the first name of your best friend at kindergarten?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::name(),
        )
    }
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn q15() -> Self { Self::friend_kindergarten() }

    /// ❗️ NOT PRODUCTION READY YET ❗️
    /// An NON-entropy-analyzed security question
    ///
    /// [Suggested question in spreadsheet][sheet], linked to [from].
    ///
    /// [from]: https://goodsecurityquestions.com/examples/
    /// [sheet]: https://docs.google.com/spreadsheets/d/1Mzg60sJYLzUzCJhe-_brprx-KRolvLclcykf4H4hF-c/edit#gid=0
    ///
    /// ❗️ NOT PRODUCTION READY YET ❗️
    pub fn street_age8() -> Self {
        Self::freeform_with_id(
            16,
            "What was the name of the street where you were living when you were 8 years old?",
            Security_NOT_PRODUCTION_READY_ExpectedAnswerFormat::with_details("<STREET NAME WITHOUT NUMBER>", "Abbey Road", ["Bad if you lived in many places during that year."]),
        )
    }
        /// ❗️ NOT PRODUCTION READY YET ❗️
        pub fn q16() -> Self { Self::street_age8() }
}

impl Security_NOT_PRODUCTION_READY_Question {
    pub fn all() -> IndexSet<Self> {
        IndexSet::<Security_NOT_PRODUCTION_READY_Question>::from_iter([
            Self::q00(),
            Self::q01(),
            Self::q02(),
            Self::q03(),
            Self::q04(),
            Self::q05(),
            Self::q06(),
            Self::q07(),
            Self::q08(),
            Self::q09(),
            Self::q10(),
            Self::q11(),
            Self::q12(),
            Self::q13(),
            Self::q14(),
            Self::q15(),
            Self::q16(),
        ])
    }
}

impl HasSampleValues for Security_NOT_PRODUCTION_READY_Question {
    /// A sample used to facilitate unit tests.
    fn sample() -> Self {
        Self::stuffed_animal()
    }

    /// A sample used to facilitate unit tests.
    fn sample_other() -> Self {
        Self::first_kiss_location()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[allow(clippy::upper_case_acronyms)]
    type SUT = Security_NOT_PRODUCTION_READY_Question;

    #[test]
    fn equality() {
        assert_eq!(SUT::sample(), SUT::sample());
        assert_eq!(SUT::sample_other(), SUT::sample_other());
    }

    #[test]
    fn inequality() {
        assert_ne!(SUT::sample(), SUT::sample_other());
    }

    #[test]
    fn test_id() {
        let test = |t: (usize, SUT)| {
            let id = t.0 as u16;
            let sut = t.1;
            assert_eq!(sut.id(), id);
        };
        let all = SUT::all();
        all.into_iter().enumerate().for_each(test);
    }
}
