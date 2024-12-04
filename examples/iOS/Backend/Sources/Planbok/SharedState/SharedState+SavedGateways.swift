import ComposableArchitecture
import Foundation
import Sargon

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static var savedGateways: Self {
		.sharedSavedGateways
	}
}

extension PersistenceKeyDefault<SargonKey<SavedGateways>> {
	public static let sharedSavedGateways = Self(
		SargonKey(
			mapping: { try $0.gateways },
			fetchIf: \.affectsSavedGateways
		),
		.default
	)
}
