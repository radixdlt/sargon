extension Data {
	public static func random(byteCount: Int) -> Self {
		var data = Data(repeating: 0, count: byteCount)
		data.withUnsafeMutableBytes {
			assert($0.count == byteCount)
			$0.initializeWithRandomBytes(count: byteCount)
		}
		return data
	}
}
