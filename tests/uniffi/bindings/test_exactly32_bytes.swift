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

extension Exactly32Bytes {
    init(data: Data) throws {
        self = try newExactly32Bytes(bytes: data)
    }
}

func test() throws {
    let bytes = try Data.random(byteCount: 32)
    let hex32 = try Exactly32Bytes(data: bytes)
    assert(hex32.bagOfBytes == bytes)
}

try! test()
