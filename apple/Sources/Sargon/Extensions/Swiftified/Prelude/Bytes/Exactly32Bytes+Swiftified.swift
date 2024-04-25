import Foundation
import SargonUniFFI

extension Exactly32Bytes: ExactlyNBytesProtocol  {
	public static let length = 32
}

extension Exactly32Bytes: SargonStringCodable {}
