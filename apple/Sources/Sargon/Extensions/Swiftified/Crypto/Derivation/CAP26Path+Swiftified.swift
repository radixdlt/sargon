import SargonUniFFI

public typealias CAP26Path = Cap26Path

// MARK: SargonModel, CAP26PathProtocol
extension CAP26Path: SargonModel, CAP26PathProtocol {
	public var path: HDPath {
		asDerivationPath.path
	}

	public var asGeneral: CAP26Path {
		self
	}
}
