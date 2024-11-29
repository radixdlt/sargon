import Foundation
import SargonUniFFI

// MARK: - AuthorizedPersonaDetailed + SargonModel
extension AuthorizedPersonaDetailed: SargonModel {}

// MARK: - AuthorizedPersonaDetailed + Identifiable
extension AuthorizedPersonaDetailed: Identifiable {
	public typealias ID = IdentityAddress
	public var id: ID {
		identityAddress
	}
}
