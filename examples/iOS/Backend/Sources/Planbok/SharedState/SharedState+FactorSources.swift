import Foundation
import Sargon
import SargonUniFFI
import ComposableArchitecture
import IdentifiedCollections

public typealias FactorSources = IdentifiedArrayOf<FactorSource>

public typealias Shield = SecurityStructureConfigurationReference
public typealias Shields = IdentifiedArrayOf<Shield>

extension FactorSources {
	public func compactMap<F>(as kind: F.Type = F.self) -> IdentifiedArrayOf<F> where F: FactorSourceProtocol {
		self.elements.compactMap({ $0.extract(F.self) }).asIdentified()
	}
	public func filter(kind: FactorSourceKind) -> Self {
		self.elements.filter({ $0.kind == kind }).asIdentified()
	}
}


public typealias DeviceFactorSources = IdentifiedArrayOf<DeviceFactorSource>
public typealias LedgerHWWalletFactorSources = IdentifiedArrayOf<LedgerHardwareWalletFactorSource>
public typealias ArculusCardFactorSources = IdentifiedArrayOf<ArculusCardFactorSource>
public typealias OffDeviceMnemonicFactorSources = IdentifiedArrayOf<OffDeviceMnemonicFactorSource>
public typealias SecurityQuestionsFactorSources = IdentifiedArrayOf<SecurityQuestionsNotProductionReadyFactorSource>

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

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<Shields>> {
	public static var shields: Self {
		Self.sharedShields
	}
}
extension PersistenceKeyDefault<SargonKey<Shields>> {
	public static let sharedShields = Self(
		SargonKey(
			accessing: \.shields,
			fetchIf: \.affectsShields
		),
		[]
	)
}

extension SargonOS {
    
    public var factorSources: FactorSources {
        factorSources().asIdentified()
    }
	
	
	public var shields: Shields {
		securityStructureConfigurationReferences().asIdentified()
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
