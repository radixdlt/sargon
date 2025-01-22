pub trait IsPathComponentStringConvertible {
    const VERBOSE_SYNTAX_SUFFIX: &'static str;
    const SHORTHAND_SYNTAX_SUFFIX: &'static str;
    const ACCEPTABLE_SUFFIXES: [&'static str; 2] =
        [Self::VERBOSE_SYNTAX_SUFFIX, Self::SHORTHAND_SYNTAX_SUFFIX];
}
