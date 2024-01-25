import radix_wallet_kit
import Foundation

extension Data {
	public static func random(byteCount: Int) throws -> Self {
		var bytes = [UInt8](repeating: 0, count: byteCount)
		let status = SecRandomCopyBytes(kSecRandomDefault, byteCount, &bytes)
		if status == errSecSuccess {
			return Self(bytes)
		}
		struct UnableToGenerateBytes: Swift.Error {}
		throw UnableToGenerateBytes()
	}
}
extension RadixConnectPassword {
    init(bytes: Hex32Bytes) {
        self = newRadixConnectPassword(bytes: bytes)
    }
    init() throws {
        try self.init(bytes: Hex32Bytes(bytes: Data.random(byteCount: 32)))
    }
}

func test() throws {
    let bytes = try Data.random(byteCount: 32)
    let password = RadixConnectPassword(bytes: bytes)
    assert(password.value == bytes)
}

try! test()
