import SargonUniFFI

extension SLIP10Curve {
	public init(_ string: String) throws {
		self = try newSlip10CurveFromString(curve: string)
	}
	
	public func toString() -> String {
		slip10CurveToString(curve: self)
	}
}

