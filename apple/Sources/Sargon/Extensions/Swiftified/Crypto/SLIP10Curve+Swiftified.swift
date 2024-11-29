import SargonUniFFI

public typealias SLIP10Curve = Slip10Curve

// MARK: - SLIP10Curve + SargonModel
extension SLIP10Curve: SargonModel {}

// MARK: - SLIP10Curve + Identifiable
extension SLIP10Curve: Identifiable {
	public typealias ID = String
	public var id: ID {
		toString()
	}
}

// MARK: - SLIP10Curve + CustomStringConvertible
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

// MARK: - SLIP10Curve + SargonStringCodable
extension SLIP10Curve: SargonStringCodable {}
