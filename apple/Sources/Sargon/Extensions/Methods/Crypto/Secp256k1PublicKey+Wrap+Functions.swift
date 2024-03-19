// MARK: Initializers
extension Secp256k1PublicKey {
	public init(hex: String) throws {
		// Rust: `new_secp256k1_public_key_from_hex`
		self = try newSecp256k1PublicKeyFromHex(hex: hex)
	}
	
	public init(bytes: some DataProtocol) throws {
		// Rust: `new_secp256k1_public_key_from_bytes`
		self = try newSecp256k1PublicKeyFromBytes(bytes: Data(bytes))
	}
}

// MARK: Func -> Method / Computed Prop
extension Secp256k1PublicKey {
	public var hex: String {
		// Rust: `secp256k1_public_key_to_hex`
		secp256k1PublicKeyToHex(publicKey: self)
	}
	
	public var data: Data {
		// Rust: `secp256k1_public_key_to_bytes`
		secp256k1PublicKeyToBytes(publicKey: self)
	}
}

#if DEBUG
// MARK: Sample Values
extension Secp256k1PublicKey {
	public static let sample: Self = newSecp256k1PublicKeySample()
	public static let sampleOther: Self = newSecp256k1PublicKeySampleOther()
}
#endif // DEBUG
