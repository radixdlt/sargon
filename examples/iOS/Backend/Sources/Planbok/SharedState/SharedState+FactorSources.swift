import ComposableArchitecture
import Foundation
import IdentifiedCollections
import Sargon
import SargonUniFFI

public typealias FactorSources = IdentifiedArrayOf<FactorSource>

public typealias Shield = SecurityStructureOfFactorSources
public typealias Shields = IdentifiedArrayOf<Shield>

public typealias ShieldReference = SecurityStructureOfFactorSourceIDs
public typealias ShieldReferences = IdentifiedArrayOf<ShieldReference>

extension FactorSources {
	public func compactMap<F: FactorSourceProtocol>(as kind: F.Type = F.self) -> IdentifiedArrayOf<F> {
		self.elements.compactMap { $0.extract(F.self) }.asIdentified()
	}

	public func filter(kind: FactorSourceKind) -> Self {
		self.elements.filter { $0.kind == kind }.asIdentified()
	}
}

public typealias DeviceFactorSources = IdentifiedArrayOf<DeviceFactorSource>
public typealias LedgerHWWalletFactorSources = IdentifiedArrayOf<LedgerHardwareWalletFactorSource>
public typealias ArculusCardFactorSources = IdentifiedArrayOf<ArculusCardFactorSource>
public typealias OffDeviceMnemonicFactorSources = IdentifiedArrayOf<OffDeviceMnemonicFactorSource>
public typealias SecurityQuestionsFactorSources = IdentifiedArrayOf<SecurityQuestionsNotProductionReadyFactorSource>

extension PersistenceReaderKey where Self == PersistenceKeyDefault<SargonKey<FactorSources>> {
	public static var factorSources: Self {
		sharedFactorSources
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
		sharedShields
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
		try! factorSources().asIdentified()
	}

	public var shieldReferences: ShieldReferences {
		try! securityStructuresOfFactorSourceIds().asIdentified()
	}

	public var shields: Shields {
		shieldReferences.compactMap { try? self.securityStructureOfFactorSourcesFromSecurityStructureOfFactorSourceIds(structureOfIds: $0) }.asIdentified()
	}

	public var deviceFactorSources: DeviceFactorSources {
		factorSources.compactMap { $0.extract(DeviceFactorSource.self) }.asIdentified()
	}

	public var ledgerFactorSources: LedgerHWWalletFactorSources {
		factorSources.compactMap { $0.extract(LedgerHardwareWalletFactorSource.self) }.asIdentified()
	}

	public var arculusCardFactorSources: ArculusCardFactorSources {
		factorSources.compactMap { $0.extract(ArculusCardFactorSource.self) }.asIdentified()
	}

	public var offDeviceMnemonicFactorSources: OffDeviceMnemonicFactorSources {
		factorSources.compactMap { $0.extract(OffDeviceMnemonicFactorSource.self) }.asIdentified()
	}

	public var securityQuestionsFactorSources: SecurityQuestionsFactorSources {
		factorSources.compactMap { $0.extract(SecurityQuestionsNotProductionReadyFactorSource.self) }.asIdentified()
	}
}

extension EventKind {
	public var affectsFactorSources: Bool {
		eventKindAffectsFactorSources(eventKind: self)
	}

	public var affectsShields: Bool {
		eventKindAffectsSecurityStructures(eventKind: self)
	}
}
