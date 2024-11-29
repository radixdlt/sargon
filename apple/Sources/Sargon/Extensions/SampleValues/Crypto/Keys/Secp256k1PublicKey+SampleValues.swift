import SargonUniFFI

#if DEBUG

// MARK: Sample Values
extension Secp256k1PublicKey {
	public static let sample: Self = newSecp256k1PublicKeySample()
	public static let sampleOther: Self = newSecp256k1PublicKeySampleOther()
}
#endif // DEBUG
