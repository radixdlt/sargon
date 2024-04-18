import SargonUniFFI

#if DEBUG
extension SLIP10Curve {
	public static let sample: Self = .curve25519
	public static let sampleOther: Self = .secp256k1
}
#endif // DEBUG
