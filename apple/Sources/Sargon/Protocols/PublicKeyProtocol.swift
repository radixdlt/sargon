public protocol PublicKeyProtocol: SargonModel, CustomStringConvertible {
	init(hex: String) throws
	init(bytes: some DataProtocol) throws

	var data: Data { get }
	var hex: String { get }
}

extension PublicKeyProtocol {
	public var description: String {
		hex
	}
}
