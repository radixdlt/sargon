//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-05-26.
//

import Foundation
import Sargon
import ComposableArchitecture

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<AccountsForDisplay>> {
	public static var accountsForDisplay: Self {
		Self.sharedAccountsForDisplay
	}
}

extension PersistenceKeyDefault<SargonKey<AccountsForDisplay>> {
	public static let sharedAccountsForDisplay = Self(
		SargonKey(
			accessing: \.accountsForDisplayOnCurrentNetworkIdentified,
			fetchIf: \.affectsCurrentAccounts
		),
		.default
	)
}
