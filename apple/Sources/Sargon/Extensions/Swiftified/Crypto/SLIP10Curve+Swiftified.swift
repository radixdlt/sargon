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

extension SLIP10Curve {
	public init?(rawValue: String) {
		try? self.init(rawValue)
	}
}

extension SLIP10Curve: SargonStringCodable {}
