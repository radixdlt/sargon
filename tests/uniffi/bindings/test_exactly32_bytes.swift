import Foundation
import Sargon

enum ByteHexEncodingErrors: Error {
	case incorrectHexValue
	case incorrectString
}

let charA = UInt8(UnicodeScalar("a").value)
let char0 = UInt8(UnicodeScalar("0").value)

private func htoi(_ value: UInt8) throws -> UInt8 {
	switch value {
	case char0...char0 + 9:
		return value - char0
	case charA...charA + 5:
		return value - charA + 10
	default:
		throw ByteHexEncodingErrors.incorrectHexValue
	}
}

private func itoh(_ value: UInt8) -> UInt8 {
	return (value > 9) ? (charA + value - 10) : (char0 + value)
}

extension DataProtocol {
	var hex: String {
		let hexLen = self.count * 2
		var hexChars = [UInt8](repeating: 0, count: hexLen)
		var offset = 0

		self.regions.forEach { (_) in
			for i in self {
				hexChars[Int(offset * 2)] = itoh((i >> 4) & 0xF)
				hexChars[Int(offset * 2 + 1)] = itoh(i & 0xF)
				offset += 1
			}
		}

		return String(bytes: hexChars, encoding: .utf8)!
	}
}

extension Data {
	init(hex hexString: String) throws {
		self.init()

		if hexString.count % 2 != 0 || hexString.count == 0 {
			throw ByteHexEncodingErrors.incorrectString
		}

		let stringBytes: [UInt8] = Array(
			hexString.lowercased().data(using: String.Encoding.utf8)!)

		for i in stride(from: stringBytes.startIndex, to: stringBytes.endIndex - 1, by: 2) {
			let char1 = stringBytes[i]
			let char2 = stringBytes[i + 1]

			try self.append(htoi(char1) << 4 + htoi(char2))
		}
	}
}


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
	let bytes = Data.random(byteCount: 32)
	let exactly32Bytes = try Exactly32Bytes(data: bytes)
	assert(exactly32BytesToBytes(bytes: exactly32Bytes) == bytes)
}

try! test()
