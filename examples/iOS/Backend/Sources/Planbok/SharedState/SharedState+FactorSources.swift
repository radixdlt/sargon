import Foundation
import Sargon
import SargonUniFFI
import ComposableArchitecture
import IdentifiedCollections

public typealias FactorSources = IdentifiedArrayOf<FactorSource>
public typealias DeviceFactorSources = IdentifiedArrayOf<DeviceFactorSource>
public typealias LedgerHWWalletFactorSources = IdentifiedArrayOf<LedgerHardwareWalletFactorSource>
public typealias ArculusCardFactorSources = IdentifiedArrayOf<ArculusCardFactorSource>
public typealias OffDeviceMnemonicFactorSources = IdentifiedArrayOf<OffDeviceMnemonicFactorSource>
public typealias SecurityQuestionsFactorSources = IdentifiedArrayOf<SecurityQuestionsNotProductionReadyFactorSource>









extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<OffDeviceMnemonicFactorSources>> {
	public static var offDeviceMnemonicFactorSources: Self {
		Self.sharedOffDeviceMnemonicFactorSources
	}
}

extension PersistenceKeyDefault<SargonKey<OffDeviceMnemonicFactorSources>> {
	public static let sharedOffDeviceMnemonicFactorSources = Self(
		SargonKey(
			accessing: \.offDeviceMnemonicFactorSources,
			fetchIf: \.affectsFactorSources
		),
		[]
	)
}


extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<SecurityQuestionsFactorSources>> {
	public static var securityQuestionsFactorSources: Self {
		Self.sharedSecurityQuestions
	}
}

extension PersistenceKeyDefault<SargonKey<SecurityQuestionsFactorSources>> {
	public static let sharedSecurityQuestions = Self(
		SargonKey(
			accessing: \.securityQuestionsFactorSources,
			fetchIf: \.affectsFactorSources
		),
		[]
	)
}






extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<DeviceFactorSources>> {
	public static var deviceFactorSources: Self {
		Self.sharedDeviceFactorSources
	}
}

extension PersistenceKeyDefault<SargonKey<DeviceFactorSources>> {
	public static let sharedDeviceFactorSources = Self(
		SargonKey(
			accessing: \.deviceFactorSources,
			fetchIf: \.affectsFactorSources
		),
		[]
	)
}




extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<ArculusCardFactorSources>> {
	public static var arculusFactorSources: Self {
		Self.sharedArculusFactorSources
	}
}

extension PersistenceKeyDefault<SargonKey<ArculusCardFactorSources>> {
	public static let sharedArculusFactorSources = Self(
		SargonKey(
			accessing: \.arculusCardFactorSources,
			fetchIf: \.affectsFactorSources
		),
		[]
	)
}


extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<LedgerHWWalletFactorSources>> {
	public static var ledgerFactorSources: Self {
		Self.sharedLedgerFactorSources
	}
}

extension PersistenceKeyDefault<SargonKey<LedgerHWWalletFactorSources>> {
	public static let sharedLedgerFactorSources = Self(
		SargonKey(
			accessing: \.ledgerFactorSources,
			fetchIf: \.affectsFactorSources
		),
		[]
	)
}


extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<FactorSources>> {
    public static var factorSources: Self {
        Self.sharedFactorSources
    }
}

extension PersistenceKeyDefault<SargonKey<FactorSources>> {
    public static let sharedFactorSources = Self(
        SargonKey(
            accessing: \.factorSources,
            fetchIf: \.affectsFactorSources
        ),
		[]
    )
}

extension SargonOS {
    
    public var factorSources: FactorSources {
        factorSources().asIdentified()
    }
    
	public var deviceFactorSources: DeviceFactorSources {
		factorSources.compactMap({ $0.extract(DeviceFactorSource.self)}).asIdentified()
	}
	
	public var ledgerFactorSources: LedgerHWWalletFactorSources {
		factorSources.compactMap({ $0.extract(LedgerHardwareWalletFactorSource.self)}).asIdentified()
	}
	
	public var arculusCardFactorSources: ArculusCardFactorSources {
		factorSources.compactMap({ $0.extract(ArculusCardFactorSource.self)}).asIdentified()
	}
	
	public var offDeviceMnemonicFactorSources: OffDeviceMnemonicFactorSources {
		factorSources.compactMap({ $0.extract(OffDeviceMnemonicFactorSource.self)}).asIdentified()
	}
	
	public var securityQuestionsFactorSources: SecurityQuestionsFactorSources {
		factorSources.compactMap({ $0.extract(SecurityQuestionsNotProductionReadyFactorSource.self)}).asIdentified()
	}
}

extension EventKind {
    
    public var affectsFactorSources: Bool {
        eventKindAffectsFactorSources(eventKind: self)
    }
}
