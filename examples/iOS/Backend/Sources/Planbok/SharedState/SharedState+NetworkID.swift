import ComposableArchitecture
import Dependencies
import Foundation
import Sargon

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static var network: Self {
		.sharedNetwork
	}
}

extension PersistenceKeyDefault<SargonKey<NetworkID>> {
	public static let sharedNetwork = Self(
		SargonKey(
			mapping: { try $0.currentNetworkID },
			fetchIf: \.affectsCurrentNetwork
		),
		.default
	)
}
