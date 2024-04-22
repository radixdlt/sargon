import SargonUniFFI

extension SignatureWithPublicKey {
	public var signature: Signature {
		signatureWithPublicKeyGetSignature(signatureWithPublicKey: self)
	}
	
	public var publicKey: PublicKey {
		signatureWithPublicKeyGetPublicKey(signatureWithPublicKey: self)
	}
	
	public func isValid(_ hash: Hash) -> Bool {
        signatureWithPublicKeyIsValid(signatureWithPublicKey: self, forHash: hash)
	}
}
