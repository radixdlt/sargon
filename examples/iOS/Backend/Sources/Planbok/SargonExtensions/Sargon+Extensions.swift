//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-26.
//

import Foundation
import Sargon

public typealias AccountsForDisplay = IdentifiedArrayOf<AccountForDisplay>

extension IdentifiedArray where Element: Identifiable, Element.ID == ID {
	public static var `default`: Self { IdentifiedArrayOf.init() }
}

extension NetworkID {
	public static let `default` = Self.mainnet
}

extension SargonOS {
	
	public var accountsForDisplayOnCurrentNetworkIdentified: AccountsForDisplay {
		accountsForDisplayOnCurrentNetwork.asIdentified()
	}
	
}
