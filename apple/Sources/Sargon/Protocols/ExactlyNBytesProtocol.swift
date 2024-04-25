import SargonUniFFI
import Foundation

public protocol ExactlyNBytesProtocol: BinaryProtocol {
	static var length: Int { get }
}

extension ExactlyNBytesProtocol {
	public static func generate() -> Self {
		try! Self.init(bytes: Data.random(byteCount: Self.length))
	}
}
