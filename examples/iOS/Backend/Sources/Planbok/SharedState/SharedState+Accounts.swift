import ComposableArchitecture
import Foundation
import Sargon

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<AccountsForDisplay>> {
	public static var accountsForDisplay: Self {
		sharedAccountsForDisplay
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
