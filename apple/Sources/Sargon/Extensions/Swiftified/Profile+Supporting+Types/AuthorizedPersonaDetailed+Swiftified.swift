//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-28.
//

import Foundation
import SargonUniFFI

extension AuthorizedPersonaDetailed: SargonModel {}
extension AuthorizedPersonaDetailed: Identifiable {
	public typealias ID = IdentityAddress
	public var id: ID {
		identityAddress
	}
}
