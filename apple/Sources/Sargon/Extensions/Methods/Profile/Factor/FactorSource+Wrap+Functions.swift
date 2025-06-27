import Foundation
import SargonUniFFI

extension FactorSource {
	public func toString() -> String {
		factorSourceToString(factorSource: self)
	}

	public var supportsOlympia: Bool {
		factorSourceSupportsOlympia(factorSource: self)
	}

	public var supportsBabylon: Bool {
		factorSourceSupportsBabylon(factorSource: self)
	}

	public var name: String {
		factorSourceName(factorSource: self)
	}

	public func spotCheck(input: SpotCheckInput) -> Bool {
		factorSourcePerformSpotCheck(factorSource: self, input: input)
	}
}
