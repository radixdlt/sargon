#if DEBUG
extension Secp256k1Signature {
	public static let sample: Self = newSecp256k1SignatureSample()
	public static let sampleOther: Self = newSecp256k1SignatureSampleOther()
}
#endif // DEBUG
