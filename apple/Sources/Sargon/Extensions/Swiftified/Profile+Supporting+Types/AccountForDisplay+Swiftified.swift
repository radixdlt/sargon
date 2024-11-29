import Foundation
import SargonUniFFI

// MARK: - AccountForDisplay + SargonModel
extension AccountForDisplay: SargonModel {}

// MARK: - AccountForDisplay + Identifiable
extension AccountForDisplay: Identifiable {
	public typealias ID = AccountAddress
	public var id: ID {
		address
	}
}
