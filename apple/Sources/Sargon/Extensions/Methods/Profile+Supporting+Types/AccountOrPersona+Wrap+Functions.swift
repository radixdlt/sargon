//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-27.
//

import Foundation
import SargonUniFFI

extension AccountOrPersona {
	public var id: AddressOfAccountOrPersona {
		accountOrPersonaGetId(entity: self)
	}
}
