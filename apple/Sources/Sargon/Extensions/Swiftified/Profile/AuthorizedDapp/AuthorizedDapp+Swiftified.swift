//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-15.
//

import Foundation
import SargonUniFFI

public typealias DappDefinitionAddress = AccountAddress

extension AuthorizedDapp: SargonModel {}
extension AuthorizedDapp: SargonObjectCodable {}

#if DEBUG
extension AuthorizedDapp {
	public static let sampleValuesMainnet: [Self] = [.sampleMainnet, .sampleMainnetOther]
	public static let sampleValuesStokenet: [Self] = [.sampleStokenet, .sampleStokenetOther]
	public static let sampleValues: [Self] = Self.sampleValuesMainnet + Self.sampleValuesStokenet
}
#endif // DEBUG

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
