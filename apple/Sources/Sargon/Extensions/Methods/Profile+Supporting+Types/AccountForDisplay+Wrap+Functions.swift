import Foundation
import SargonUniFFI

extension AccountForDisplay {
	public init(_ account: Account) {
		self = newAccountForDisplayFromAccount(account: account)
	}
}
