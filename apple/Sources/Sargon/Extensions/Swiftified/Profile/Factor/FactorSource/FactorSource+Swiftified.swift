import SargonUniFFI

extension FactorSource: SargonModel {}

extension FactorSource: CustomStringConvertible {
	public var description: String {
		toString()
	}
	
}

extension FactorSource: Identifiable {
	public typealias ID = FactorSourceID
	public var id: ID {
		switch self {
		case let .device(value): value.id.asGeneral
		case let .ledger(value): value.id.asGeneral
		}
	}
}

extension FactorSource: BaseFactorSourceProtocol {
	public var factorSourceID: FactorSourceID {
		id
	}
	
	public var factorSourceKind: FactorSourceKind {
		switch self {
		case let .device(value): value.factorSourceKind
		case let .ledger(value): value.factorSourceKind
		}
	}
	
	public var common: FactorSourceCommon {
		get {
			switch self {
			case let .device(value): value.common
			case let .ledger(value): value.common
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
	
}
