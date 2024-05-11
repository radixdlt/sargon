import Foundation
import SargonUniFFI

// MARK: Initializers
extension Ed25519PublicKey {
    public init(hex: String) throws {
        // Rust: `new_ed25519_public_key_from_hex`
        self = try newEd25519PublicKeyFromHex(hex: hex)
    }
    
    public init(bytes: some DataProtocol) throws {
        // Rust: `new_ed25519_public_key_from_bytes`
        self = try newEd25519PublicKeyFromBytes(bytes: Data(bytes))
    }
    
    public init(jsonStringLiteral: String) throws {
        self = try newEd25519PublicKeyFromJsonString(jsonString: jsonStringLiteral)
    }
    
    public func jsonStringLiteral() -> String {
		ed25519PublicKeyToJsonString(ed25519PublicKey: self)
    }
}

// MARK: Func -> Method / Computed Prop
extension Ed25519PublicKey {
    public var hex: String {
        // Rust: `ed25519_public_key_to_hex`
        ed25519PublicKeyToHex(publicKey: self)
    }
    
    public var data: Data {
        // Rust: `ed25519_public_key_to_bytes`
        ed25519PublicKeyToBytes(publicKey: self)
    }
}
