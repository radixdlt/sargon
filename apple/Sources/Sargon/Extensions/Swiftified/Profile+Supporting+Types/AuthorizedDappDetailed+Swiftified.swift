//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-28.
//

import Foundation
import SargonUniFFI

extension AuthorizedDappDetailed: SargonModel {}

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
