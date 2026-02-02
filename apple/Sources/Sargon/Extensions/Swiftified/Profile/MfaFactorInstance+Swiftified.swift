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
}
