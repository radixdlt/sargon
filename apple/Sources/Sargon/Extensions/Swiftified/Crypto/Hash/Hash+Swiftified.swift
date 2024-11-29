import Foundation
import SargonUniFFI

// MARK: - Hash + SargonModel
extension Hash: SargonModel {}

extension Hash {
	public var hex: String {
		data.hex
	}

	public func hash() -> Self {
		data.hash()
	}

	public var data: Data {
		bytes.data
	}
}

// MARK: - Hash + CustomStringConvertible
extension Hash: CustomStringConvertible {
	public var description: String {
		hex
	}
}
