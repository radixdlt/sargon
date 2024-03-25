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
    
    /// Returns the public key on **compressed** form (33 bytes) as
    /// a hexadecimal string (66 chars).
    ///
    /// You can use `uncompressedData.hex` for uncompressed hex string.
	public var hex: String {
		// Rust: `secp256k1_public_key_to_hex`
		secp256k1PublicKeyToHex(publicKey: self)
	}
    
    /// Returns the key on **compressed** form (33 bytes)
    ///
    /// Use `uncompressedData` for uncompressed format (65 bytes)
    public var data: Data {
        compressedData
    }
	
    /// Returns the key on **compressed** form (33 bytes)
    ///
    /// Use `uncompressedData` for uncompressed format (65 bytes)
	public var compressedData: Data {
		// Rust: `secp256k1_public_key_to_bytes`
		secp256k1PublicKeyToBytes(publicKey: self)
	}
    
    /// Returns the key on **uncompressed** form (65 bytes)
    ///
    /// Use `compressedData` for compressed format (33 bytes)
    public var uncompressedData: Data {
        // Rust: `secp256k1_public_key_to_bytes_uncompressed`
        secp256k1PublicKeyToBytesUncompressed(publicKey: self)
    }
}
