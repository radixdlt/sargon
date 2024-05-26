import Sargon
import Dependencies
import Foundation
import ComposableArchitecture

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static var network: Self {
		Self.sharedNetwork
	}
}

extension PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static let sharedNetwork = Self(
		SargonKey(
			accessing: \.currentNetworkID,
			fetchIf: \.affectsCurrentNetwork
		),
		.default
	)
}

