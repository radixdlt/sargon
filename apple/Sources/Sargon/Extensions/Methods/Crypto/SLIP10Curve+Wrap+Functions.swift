import SargonUniFFI

extension SLIP10Curve {
	public init(_ string: String) throws {
		self = try newSlip10CurveFromString(curve: string)
	}

	public func toString() -> String {
		slip10CurveToString(curve: self)
	}

	/// SLIP10Curve -> SargonStringCodable
	public init(jsonStringLiteral: String) throws {
		self = try newSLIP10CurveFromJsonString(jsonString: jsonStringLiteral)
	}

	public func jsonStringLiteral() -> String {
		sLIP10CurveToJsonString(sLIP10Curve: self)
	}
}
