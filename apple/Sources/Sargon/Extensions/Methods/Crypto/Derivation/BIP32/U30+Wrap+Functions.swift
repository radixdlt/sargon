import SargonUniFFI

extension U30 {
	public init(value: UInt32) throws {
		self = try newU30(value: value)
	}

	public var value: UInt32 {
		u30GetValue(u30: self)
	}
}
