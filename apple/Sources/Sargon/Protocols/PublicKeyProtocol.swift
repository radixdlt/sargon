#if DEBUG
public protocol BasePublicKeyProtocol: SargonModel, ExpressibleByStringLiteral {}
#else
public protocol BasePublicKeyProtocol: SargonModel {}
#endif // DEBUG

public protocol PublicKeyProtocol: BasePublicKeyProtocol, CustomStringConvertible {
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

#if DEBUG
extension PublicKeyProtocol {
	public init(stringLiteral value: String) {
		self = try! Self(hex: value)
	}
}
#endif // DEBUG
