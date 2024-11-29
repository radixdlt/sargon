import Foundation
import SargonUniFFI

public typealias DappDefinitionAddress = AccountAddress

// MARK: - AuthorizedDapp + SargonModel
extension AuthorizedDapp: SargonModel {}

// MARK: - AuthorizedDapp + SargonObjectCodable
extension AuthorizedDapp: SargonObjectCodable {}

#if DEBUG
extension AuthorizedDapp {
	public static let sampleValuesMainnet: [Self] = [.sampleMainnet, .sampleMainnetOther]
	public static let sampleValuesStokenet: [Self] = [.sampleStokenet, .sampleStokenetOther]
	public static let sampleValues: [Self] = Self.sampleValuesMainnet + Self.sampleValuesStokenet
}
#endif // DEBUG

// MARK: - AuthorizedDapp + Identifiable
extension AuthorizedDapp: Identifiable {
	public typealias ID = DappDefinitionAddress

	public var dAppDefinitionAddress: DappDefinitionAddress {
		dappDefinitionAddress
	}

	public var id: ID {
		dAppDefinitionAddress
	}

	public var networkID: NetworkID {
		networkId
	}
}
