import SargonUniFFI

extension Message {
	public static func plaintext(string: String) -> Self {
		newMessagePlaintextString(string: string)
	}
	
	public var plaintext: String? {
		messageAsPlaintext(message: self)
	}
}
