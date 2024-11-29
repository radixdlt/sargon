import Foundation
import SargonUniFFI

// MARK: - RadixConnectPurpose + SargonModel
extension RadixConnectPurpose: SargonModel {}

// MARK: - RadixConnectPurpose + SargonStringCodable
extension RadixConnectPurpose: SargonStringCodable {}

extension RadixConnectPurpose {
	public init(rawValue: String) {
		self.init(string: rawValue)
	}
}
