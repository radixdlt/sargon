extension IntentSignature: SargonModel {}
extension IntentSignature: CustomStringConvertible {
	public var description: String {
		signatureWithPublicKey.description
	}
}
