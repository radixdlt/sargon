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

extension FactorSource: FactorSourceProtocol {
	public var factorSourceID: FactorSourceID {
		id
	}
	
	public var factorSourceKind: SargonUniFFI.FactorSourceKind {
		switch self {
		case let .device(value): value.factorSourceKind
		case let .ledger(value): value.factorSourceKind
		}
	}
	
	public var asGeneral: FactorSource { self }
	
	public func extract<F>(_ type: F.Type = F.self) -> F? where F: FactorSourceSpecificProtocol {
		F.extract(from: self)
	}
	
	public func extract<F>(as _: F.Type = F.self) throws -> F where F: FactorSourceSpecificProtocol {
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
