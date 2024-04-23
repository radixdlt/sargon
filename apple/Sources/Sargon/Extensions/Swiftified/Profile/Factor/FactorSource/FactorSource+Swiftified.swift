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
	

}
