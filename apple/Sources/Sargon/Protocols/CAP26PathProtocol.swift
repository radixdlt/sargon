import SargonUniFFI

// MARK: - DerivationPathProtocol
public protocol DerivationPathProtocol: HDPathProtocol {
	var asDerivationPath: DerivationPath { get }
}

// MARK: - CAP26PathProtocol
public protocol CAP26PathProtocol: DerivationPathProtocol {
	var asGeneral: CAP26Path { get }
}

extension CAP26PathProtocol {
	public var asDerivationPath: DerivationPath {
		.cap26(value: asGeneral)
	}

	public func toString() -> String {
		asGeneral.toString()
	}
}

extension CAP26PathProtocol {
	public var description: String {
		toString()
	}
}
