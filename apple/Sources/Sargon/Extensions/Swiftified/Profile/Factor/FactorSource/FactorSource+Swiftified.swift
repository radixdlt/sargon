import SargonUniFFI

extension FactorSource: SargonModel {}

extension FactorSource: Identifiable {
	public typealias ID = FactorSourceID
	public var id: ID {
		switch self {
		case let .device(value): value.id.asGeneral
		case let .ledger(value): value.id.asGeneral
		}
	}
}
