extension DataProtocol {
	public func hash() -> Exactly32Bytes {
		SargonUniFFI.hash(data: Data(self))
	}
}
