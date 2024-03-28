import SargonUniFFI

extension NotarizedTransaction {
	public func compile() -> CompiledNotarizedIntent {
		notarizedTransactionCompile(notarizedTransaction: self)
	}
}
