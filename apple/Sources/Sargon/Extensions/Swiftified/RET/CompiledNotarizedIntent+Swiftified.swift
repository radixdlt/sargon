extension CompiledNotarizedIntent: SargonModel {}

extension CompiledNotarizedIntent: CustomStringConvertible {
	public var description: String {
		data.hex
	}
}
