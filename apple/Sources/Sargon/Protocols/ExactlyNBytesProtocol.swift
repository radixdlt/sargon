import Foundation
import SargonUniFFI

// MARK: - ExactlyNBytesProtocol
public protocol ExactlyNBytesProtocol: BinaryProtocol {
	static var length: Int { get }
}

extension ExactlyNBytesProtocol {
	public static func generate() -> Self {
		try! Self(bytes: Data.random(byteCount: length))
	}
}
