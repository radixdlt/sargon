extension CompiledNotarizedIntent {
	public var data: Data {
		compiledNotarizedIntentGetBytes(compiledNotarizedIntent: self)
	}
}
