import SargonUniFFI

public typealias SLIP10Curve = Slip10Curve

extension SLIP10Curve: SargonModel { }
extension SLIP10Curve: Identifiable {
	public typealias ID = String
	public var id: ID {
		toString()
	}
}

extension SLIP10Curve: CustomStringConvertible {
	public var description: String {
		toString()
	}
}
