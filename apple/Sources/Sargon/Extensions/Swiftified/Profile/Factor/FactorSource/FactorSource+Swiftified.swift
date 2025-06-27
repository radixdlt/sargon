import SargonUniFFI

// MARK: - FactorSource + SargonModel
extension FactorSource: SargonModel {}

// MARK: - FactorSource + CustomStringConvertible
extension FactorSource: CustomStringConvertible {
	public var description: String {
		toString()
	}
}

// MARK: - FactorSource + Identifiable
extension FactorSource: Identifiable {
	public typealias ID = FactorSourceID
	public var id: ID {
		switch self {
		case let .device(value): value.id.asGeneral
		case let .arculusCard(value): value.id.asGeneral
//		case let .securityQuestions(value): value.id.asGeneral
		case let .ledger(value): value.id.asGeneral
		case let .offDeviceMnemonic(value): value.id.asGeneral
//		case let .trustedContact(value): value.id.asGeneral
		case let .password(value): value.id.asGeneral
		}
	}
}

// MARK: - FactorSource + BaseFactorSourceProtocol
extension FactorSource: BaseFactorSourceProtocol {
	public var factorSourceID: FactorSourceID {
		id
	}

	public var factorSourceKind: FactorSourceKind {
		switch self {
		case let .device(value): value.factorSourceKind
		case let .ledger(value): value.factorSourceKind
//		case let .securityQuestions(value): value.factorSourceKind
		case let .arculusCard(value): value.factorSourceKind
		case let .offDeviceMnemonic(value): value.factorSourceKind
//		case let .trustedContact(value): value.factorSourceKind
		case let .password(value): value.factorSourceKind
		}
	}

	public var common: FactorSourceCommon {
		get {
			switch self {
			case let .device(value): value.common
			case let .ledger(value): value.common
//			case let .securityQuestions(value): value.common
			case let .arculusCard(value): value.common
			case let .offDeviceMnemonic(value): value.common
//			case let .trustedContact(value): value.common
			case let .password(value): value.common
			}
		}
		set {
			switch self {
			case var .device(source):
				source.common = newValue
				self = .device(value: source)
			case var .ledger(source):
				source.common = newValue
				self = .ledger(value: source)
			case var .offDeviceMnemonic(source):
				source.common = newValue
				self = .offDeviceMnemonic(value: source)
			case var .arculusCard(source):
				source.common = newValue
				self = .arculusCard(value: source)
//			case var .securityQuestions(source):
//				source.common = newValue
//				self = .securityQuestions(value: source)
//			case var .trustedContact(source):
//				source.common = newValue
//				self = .trustedContact(value: source)
			case var .password(source):
				source.common = newValue
				self = .password(value: source)
			}
		}
	}

	public var asGeneral: FactorSource { self }

	public func extract<F>(_ type: F.Type = F.self) -> F? where F: FactorSourceProtocol {
		F.extract(from: self)
	}

	public func extract<F>(as _: F.Type = F.self) throws -> F where F: FactorSourceProtocol {
		guard let extracted = extract(F.self) else {
			throw IncorrectFactorSourceType(
				expectedKind: F.kind,
				actualKind: factorSourceKind
			)
		}
		return extracted
	}

	public struct IncorrectFactorSourceType: Swift.Error {
		public let expectedKind: FactorSourceKind
		public let actualKind: FactorSourceKind
	}

	public var asDevice: DeviceFactorSource? {
		extract()
	}

	public var asLedger: LedgerHardwareWalletFactorSource? {
		extract()
	}

	public var asArculus: ArculusCardFactorSource? {
		extract()
	}

	public var asOffDeviceMnemonic: OffDeviceMnemonicFactorSource? {
		extract()
	}

//	public var asSecurityQuestions: SecurityQuestionsNotProductionReadyFactorSource? {
//		extract()
//	}
//
//	public var asTrustedContact: TrustedContactFactorSource? {
//		extract()
//	}

	public var asPassword: PasswordFactorSource? {
		extract()
	}
}
