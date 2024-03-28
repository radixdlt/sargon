import SargonUniFFI

#if DEBUG
extension Ed25519Signature {
	public static let sample: Self = newEd25519SignatureSample()
	public static let sampleOther: Self = newEd25519SignatureSampleOther()
}
#endif // DEBUG
