import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromHash = FactorSourceIdFromHash

// MARK: - FactorSourceIDFromHash + SargonModel
extension FactorSourceIDFromHash: SargonModel {}

// MARK: - FactorSourceIDFromHash + SargonObjectCodable
extension FactorSourceIDFromHash: SargonObjectCodable {}

// MARK: - FactorSourceIDFromHash + FactorSourceIDSpecificProtocol
extension FactorSourceIDFromHash: FactorSourceIDSpecificProtocol {
	public var asGeneral: FactorSourceID {
		.hash(value: self)
	}

	public static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self? {
		guard case let .hash(id) = someFactorSourceID.asGeneral else { return nil }
		return id
	}
}
