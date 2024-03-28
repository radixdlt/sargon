import SargonUniFFI

extension NotarySignature {
	public init(signature: Signature) {
		self = newNotarySignature(signature: signature)
	}
	
	public var signature: Signature {
		notarySignatureGetSignature(notarySignature: self)
	}
}
