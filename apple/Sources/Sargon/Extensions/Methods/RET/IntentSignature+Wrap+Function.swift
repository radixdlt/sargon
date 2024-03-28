import SargonUniFFI

extension IntentSignature {
    public init(signatureWithPublicKey: SignatureWithPublicKey) {
        self = newIntentSignatureFromSignatureWithPublicKey(
            signatureWithPublicKey: signatureWithPublicKey
        )
    }
    
    public var signatureWithPublicKey: SignatureWithPublicKey {
        intentSignatureGetSignatureWithPublicKey(intentSignature: self)
    }
}
