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
extension AuthorizedDapp: Identifiable {
	public typealias ID = DappDefinitionAddress
	
	public var dAppDefinitionAddress: DappDefinitionAddress {
		dappDefinitionAddress
	}
	public var id: ID {
		dAppDefinitionAddress
	}
}
