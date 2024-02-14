import Sargon
import Foundation

func randomByteArray(byteCount count: Int) -> [UInt8] {
	#if canImport(Darwin) || os(Linux) || os(Android) || os(Windows)
		var rng = SystemRandomNumberGenerator()
		return (0..<count).map { _ in rng.next() }
	#else
		fatalError("No secure random number generator on this platform.")
	#endif
}

extension Data {
	public static func random(byteCount: Int) -> Self {
		Data(randomByteArray(byteCount: byteCount))
	}
}

extension RadixConnectPassword {
    init(bytes: Hex32Bytes) {
        self = newRadixConnectPassword(bytes: bytes)
    }
}

func test() throws {
    let data = try Data.random(byteCount: 32)
    let bytes = try newHex32BytesFrom(bytes: data)
    let password = RadixConnectPassword(bytes: bytes)
    assert(password.value == bytes)
}

try! test()
