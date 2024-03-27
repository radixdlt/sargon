extension SignatureWithPublicKey {
	public var signature: Signature {
		signatureWithPublicKeyGetSignature(signatureWithPublicKey: self)
	}
	
	public var publicKey: PublicKey {
		signatureWithPublicKeyGetPublicKey(signatureWithPublicKey: self)
	}
}
