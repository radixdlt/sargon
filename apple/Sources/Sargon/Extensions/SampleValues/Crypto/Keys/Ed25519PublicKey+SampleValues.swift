import SargonUniFFI

#if DEBUG

// MARK: Sample Values
extension Ed25519PublicKey {
	public static let sample: Self = newEd25519PublicKeySample()
	public static let sampleOther: Self = newEd25519PublicKeySampleOther()
}
#endif // DEBUG
