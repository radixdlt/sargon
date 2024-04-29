import SargonUniFFI

public typealias SLIP10Curve = Slip10Curve

// MARK: SargonModel
extension SLIP10Curve: SargonModel {}

// MARK: Identifiable
extension SLIP10Curve: Identifiable {
	public typealias ID = String
	public var id: ID {
		toString()
	}
}

// MARK: CustomStringConvertible
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

// MARK: SargonStringCodable
extension SLIP10Curve: SargonStringCodable {}
