/// A version of an algorithm so that we can change the implementation between
/// app releases and remain backwards compatible.
pub trait VersionOfAlgorithm {
    /// Some version type of this algorithm, typically an enum with known
    /// versions.
    type Version;

    /// Version of this algorithm, used to determine which implementation
    /// to use.
    fn version(&self) -> Self::Version;

    /// A *hint* of what the algorithm does, SHOULD NOT be used
    /// for decoding or taking any decision, is primarily used
    /// as hint in JSON encoded data.
    fn description(&self) -> String;
}
