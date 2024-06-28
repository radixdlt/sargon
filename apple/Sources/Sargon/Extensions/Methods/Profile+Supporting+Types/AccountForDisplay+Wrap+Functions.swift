//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-16.
//

import Foundation
import SargonUniFFI

extension AccountForDisplay {
	
	public init(_ account: Account) {
		self = newAccountForDisplayFromAccount(account: account)
	}
}
