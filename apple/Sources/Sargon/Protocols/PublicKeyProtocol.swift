public protocol PublicKeyProtocol: SargonModel {
	init(hex: String) throws
	init(bytes: some DataProtocol) throws

	var data: Data { get }
	var hex: String { get }
}

extension Ed25519PublicKey: PublicKeyProtocol {}
extension Secp256k1PublicKey: PublicKeyProtocol {}

extension PublicKeyProtocol {
	public var description: String {
		hex
	}
}
