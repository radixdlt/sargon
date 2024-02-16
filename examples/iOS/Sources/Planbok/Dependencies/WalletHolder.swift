//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-02-16.
//

import Foundation

public struct WalletHolder: Equatable, Sendable {
	public static func == (lhs: Self, rhs: Self) -> Bool {
		lhs.wallet === rhs.wallet
	}
	
	public let wallet: Wallet
}
