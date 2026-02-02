import Foundation
import SargonUniFFI

// MARK: - MfaFactorInstance + SargonModel
extension MfaFactorInstance: SargonModel {}

// MARK: - MfaFactorInstance + SargonObjectCodable
extension MfaFactorInstance: SargonObjectCodable {}

// MARK: - MfaFactorInstance + Identifiable
extension MfaFactorInstance: Identifiable {
	public typealias ID = FactorInstance
	public var id: ID {
		factorInstance
	}

	public func nonFungibleGlobalId() throws -> NonFungibleGlobalID {
		switch factorInstance.badge {
		case let .virtual(value):
			switch value {
			case let .hierarchicalDeterministic(value):
				try value.nonFungibleGlobalId()
			}
		}
	}
}
