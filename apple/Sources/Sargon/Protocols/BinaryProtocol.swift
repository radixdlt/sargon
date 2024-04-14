import Foundation
import SargonUniFFI

#if DEBUG
public protocol BaseBinaryProtocol: SargonModel, ExpressibleByStringLiteral, ExpressibleByArrayLiteral {}
#else
public protocol BaseBinaryProtocol: SargonModel {}
#endif

public protocol BinaryProtocol: BaseBinaryProtocol, CustomStringConvertible {
	associatedtype Digest
	init(hex: String) throws
	init(bytes: some DataProtocol) throws
	
	var data: Data { get }
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

extension BinaryProtocol where Digest == Exactly32Bytes {
	public func hash() -> Exactly32Bytes {
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
