import SargonUniFFI

extension U31 {
	public init(value: UInt32) throws {
		self = try newU31(value: value)
	}

	public var value: UInt32 {
		u31GetValue(u31: self)
	}
}
