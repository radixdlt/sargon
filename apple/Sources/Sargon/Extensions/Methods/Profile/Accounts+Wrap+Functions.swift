//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-04-14.
//

import Foundation
import SargonUniFFI

extension Accounts {
	public init(_ elements: [Account]) {
		self = newAccounts(accounts: elements)
	}
	
	public var elements: [Account] {
		getAccounts(accounts: self)
	}
}
