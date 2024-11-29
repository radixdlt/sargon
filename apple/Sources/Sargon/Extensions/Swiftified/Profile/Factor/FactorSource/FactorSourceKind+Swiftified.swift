import Foundation
import SargonUniFFI

// MARK: - FactorSourceKind + SargonModel
extension FactorSourceKind: SargonModel {}

// MARK: - FactorSourceKind + CustomStringConvertible
extension FactorSourceKind: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

extension FactorSourceKind {
	public var rawValue: String {
		toString()
	}

	public init?(rawValue: String) {
		try? self.init(string: rawValue)
	}
}
