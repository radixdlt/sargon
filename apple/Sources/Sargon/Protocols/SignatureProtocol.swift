public protocol IntoSignatureProtocol {
	var signature: Signature { get }
}

public protocol SignatureProtocol: BinaryProtocol & IntoSignatureProtocol {}
