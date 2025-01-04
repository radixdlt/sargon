import SargonUniFFI

extension FactorSourceIntegrity {
	public var factorSource: FactorSource {
		switch self {
		case let .device(device):
			device.factorSource.asGeneral
		case let .ledger(ledger):
			ledger.asGeneral
		case let .arculusCard(arculus):
			arculus.asGeneral
		case let .password(password):
			password.asGeneral
		case let .offDeviceMnemonic(offDeviceMnemonic):
			offDeviceMnemonic.asGeneral
		}
	}

	public var isExplicitMain: Bool {
		switch self {
		case let .device(device):
			device.factorSource.isMainBDFS
		case .ledger, .password, .offDeviceMnemonic, .arculusCard:
			// TODO: Should we have the notion of "main" for
			// other FactorSourceKinds than device?
			false
		}
	}
}
