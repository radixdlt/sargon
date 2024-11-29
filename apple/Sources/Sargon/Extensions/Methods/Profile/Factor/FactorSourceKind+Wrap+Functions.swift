import Foundation
import SargonUniFFI

extension FactorSourceKind {
	public init(string: String) throws {
		self = try newFactorSourceKindFromString(string: string)
	}

	public func toString() -> String {
		factorSourceKindToString(kind: self)
	}
}
