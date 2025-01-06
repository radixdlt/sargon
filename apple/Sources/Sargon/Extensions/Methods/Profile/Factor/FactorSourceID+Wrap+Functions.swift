import Foundation
import SargonUniFFI

extension FactorSourceID {
	public func toString() -> String {
		factorSourceIdToString(factorSourceId: self)
	}
}
