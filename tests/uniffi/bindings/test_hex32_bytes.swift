import Sargon
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
extension Hex32Bytes {
    init(data: Data) throws {
        self = try newHex32BytesFrom(bytes: data)
    }
}

func test() throws {
    let bytes = try Data.random(byteCount: 32)
    let hex32 = try Hex32Bytes(data: bytes)
    assert(hex32.bagOfBytes == bytes)
}

try! test()
