pub trait IsPathComponentStringConvertible {
    const CANONICAL_SUFFIX: &'static str;
    const NON_CANONICAL_SUFFIXES: &'static str;
    const ACCEPTABLE_SUFFIXES: [&'static str; 2] =
        [Self::CANONICAL_SUFFIX, Self::NON_CANONICAL_SUFFIXES];
}
