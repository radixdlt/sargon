import Foundation
import SargonUniFFI

// MARK: - Exactly32Bytes + ExactlyNBytesProtocol
extension Exactly32Bytes: ExactlyNBytesProtocol {
	public static let length = 32
}

// MARK: - Exactly32Bytes + SargonStringCodable
extension Exactly32Bytes: SargonStringCodable {}
