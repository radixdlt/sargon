import Foundation
import SargonUniFFI

#if DEBUG
public protocol BaseBinaryProtocol: SargonModel, ExpressibleByStringLiteral, ExpressibleByArrayLiteral {}
#else
public protocol BaseBinaryProtocol: SargonModel {}
#endif

public protocol ToDataProtocol {
	var data: Data { get }
}
extension Hash: ToDataProtocol {}

public protocol BinaryProtocol: BaseBinaryProtocol, ToDataProtocol, CustomStringConvertible {
	associatedtype Digest: Equatable & ToDataProtocol
	init(hex: String) throws
	init(bytes: some DataProtocol) throws
	
	var hex: String { get }
	
	func hash() -> Digest
}

extension BinaryProtocol {
	
	public init(hex: String) throws {
		try self.init(bytes: Data(hex: hex))
	}
	
	public var description: String {
		hex
	}
}

extension BinaryProtocol where Digest == Hash {
	public func hash() -> Hash {
		data.hash()
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
