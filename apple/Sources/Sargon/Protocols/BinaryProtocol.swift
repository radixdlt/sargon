#if DEBUG
public protocol BaseBinaryProtocol: SargonModel, ExpressibleByStringLiteral, ExpressibleByArrayLiteral {}
#else
public protocol BaseBinaryProtocol: SargonModel {}
#endif

public protocol BinaryProtocol: BaseBinaryProtocol, CustomStringConvertible {
	init(hex: String) throws
	init(bytes: some DataProtocol) throws
	
	var data: Data { get }
	var hex: String { get }
}

extension BinaryProtocol {
	
	public init(hex: String) throws {
		try self.init(bytes: Data(hex: hex))
	}
	
	public var description: String {
		hex
	}
}

#if DEBUG
extension BinaryProtocol {
	public init(stringLiteral value: String) {
		try! self.init(hex: value)
	}
	
	public init(arrayLiteral value: UInt8...) {
		try! self.init(bytes: value)
	}
}
#endif
