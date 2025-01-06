import Foundation
import SargonUniFFI

extension FactorSourceID {
	public func toString() -> String {
		factorSourceIdToString(factorSourceId: self)
	}

	public init(jsonData: some DataProtocol) throws {
		self = try newFactorSourceIDFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		factorSourceIDsToJsonBytes(factorSourceID: self)
	}
}
