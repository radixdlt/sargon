import Foundation
import Sargon

public typealias AccountsForDisplay = IdentifiedArrayOf<AccountForDisplay>

extension IdentifiedArray where Element: Identifiable, Element.ID == ID {
	public static var `default`: Self {
		IdentifiedArrayOf()
	}
}

extension NetworkID {
	public static let `default` = Self.mainnet
}

extension SargonOS {
	public var accountsForDisplayOnCurrentNetworkIdentified: AccountsForDisplay {
		try! accountsForDisplayOnCurrentNetwork.asIdentified()
	}
}
