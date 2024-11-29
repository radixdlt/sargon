import Foundation
import SargonUniFFI

// MARK: - AuthorizedDappDetailed + SargonModel
extension AuthorizedDappDetailed: SargonModel {}

// MARK: - AuthorizedDappDetailed + Identifiable
extension AuthorizedDappDetailed: Identifiable {
	public typealias ID = DappDefinitionAddress
	public var id: ID {
		dAppDefinitionAddress
	}
}

extension AuthorizedDappDetailed {
	public var dAppDefinitionAddress: DappDefinitionAddress {
		dappDefinitionAddress
	}
}
