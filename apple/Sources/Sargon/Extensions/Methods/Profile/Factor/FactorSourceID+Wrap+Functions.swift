import Foundation
import SargonUniFFI

extension FactorSourceID {
	public func toString() -> String {
		factorSourceIdToString(factorSourceId: self)
	}

	public var kind: FactorSourceKind {
		switch self {
		case let .hash(value):
			value.kind
		}
	}

	public func spotCheck(input: SpotCheckInput) -> Bool {
		factorSourceIdPerformSpotCheck(factorSourceId: self, input: input)
	}
}
