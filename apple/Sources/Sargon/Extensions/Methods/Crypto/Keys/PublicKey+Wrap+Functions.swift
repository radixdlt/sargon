import Foundation
import SargonUniFFI

extension PublicKey {
	public init(hex: String) throws {
		self = try newPublicKeyFromHex(hex: hex)
	}
	
	public init(bytes: some DataProtocol) throws {
		self = try newPublicKeyFromBytes(bagOfBytes: Data(bytes))
	}
	
	public var data: Data {
		publicKeyToBytes(publicKey: self)
	}
	
	public var hex: String {
		publicKeyToHex(publicKey: self)
	}
	
	public func isValidSignature(
		_ intoSignature: IntoSignatureProtocol,
		for hashedMessage: Hash
	) -> Bool {
		publicKeyIsValid(
			publicKey: self,
			signature: intoSignature.signature,
			forHash: hashedMessage
		)
	}
}

