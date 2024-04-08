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
	private var profile: Profile
	public let wallet: Wallet
	public init(wallet: Wallet) {
		self.wallet = wallet
		self.profile = wallet.profile()
	}
	
	// FIXME: Replace this with an async stream of values...
	public mutating func refresh() {
		self.profile = wallet.profile()
	}
}
