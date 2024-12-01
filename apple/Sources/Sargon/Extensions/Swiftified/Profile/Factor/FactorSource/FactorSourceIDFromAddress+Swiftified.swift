import Foundation
import SargonUniFFI

public typealias FactorSourceIDFromAddress = FactorSourceIdFromAddress

// MARK: SargonModel
extension FactorSourceIDFromAddress: SargonModel {}

// MARK: SargonObjectCodable
extension FactorSourceIDFromAddress: SargonObjectCodable {}

// MARK: FactorSourceIDSpecificProtocol
extension FactorSourceIDFromAddress: FactorSourceIDSpecificProtocol {
	public var asGeneral: FactorSourceID {
		.address(value: self)
	}

	public static func extract(from someFactorSourceID: some FactorSourceIDProtocol) -> Self? {
		guard case let .address(id) = someFactorSourceID.asGeneral else { return nil }
		return id
	}
}
