import SargonUniFFI

#if DEBUG
extension NonFungibleLocalID {
    public static let sample: Self = newNonFungibleLocalIdSample()
    public static let sampleOther: Self = newNonFungibleLocalIdSampleOther()
	
	/// Generates a new `NonFungibleLocalID::Bytes` with 64 bytes.
	public static func random() -> Self {
		newNonFungibleLocalIdRandom()
	}
}
#endif
