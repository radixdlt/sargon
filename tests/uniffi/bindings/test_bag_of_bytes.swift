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

extension BagOfBytes {
	init(data: Data) {
		self = newBagOfBytesFrom(bytes: data)
	}
	static let aced = newBagOfBytesPlaceholderAced()
	static let babe = newBagOfBytesPlaceholderBabe()
	static let cafe = newBagOfBytesPlaceholderCafe()
	static let dead = newBagOfBytesPlaceholderDead()
	static let ecad = newBagOfBytesPlaceholderEcad()
	static let fade = newBagOfBytesPlaceholderFade()

	func appendingCafe() -> Self {
		bagOfBytesAppendCafe(to: self)
	}

	func appendingDeadbeef() -> Self {
		bagOfBytesAppendDeadbeef(to: self)
	}

	func prependingCafe() -> Self {
		bagOfBytesPrependCafe(inFrontOf: self)
	}

	func prependingDeadbeef() -> Self {
		bagOfBytesPrependDeadbeef(inFrontOf: self)
	}

}

func test() throws {
	var a = Data()
	var b = Data()
	assert(a == b)

	a = Data([129])
	b = BagOfBytes(data: a)
	assert(a == b)

	assert(
		try! Data(hex: "acedacedacedacedacedacedacedacedacedacedacedacedacedacedacedaced")
			== BagOfBytes.aced)
	assert(
		try! Data(hex: "babebabebabebabebabebabebabebabebabebabebabebabebabebabebabebabe")
			== BagOfBytes.babe)
	assert(
		try! Data(hex: "cafecafecafecafecafecafecafecafecafecafecafecafecafecafecafecafe")
			== BagOfBytes.cafe)

	assert(
		try! Data(hex: "deaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddeaddead")
			== BagOfBytes.dead)

	assert(
		try! Data(hex: "ecadecadecadecadecadecadecadecadecadecadecadecadecadecadecadecad")
			== BagOfBytes.ecad)

	assert(
		try! Data(hex: "fadefadefadefadefadefadefadefadefadefadefadefadefadefadefadefade")
			== BagOfBytes.fade)

	a = try! BagOfBytes(data: Data(hex: "beef"))

	assert(a.appendingCafe().hex == "beefcafe")
	assert(a.appendingDeadbeef().hex == "beefdeadbeef")
	assert(a.prependingCafe().hex == "cafebeef")
	assert(a.prependingDeadbeef().hex == "deadbeefbeef")

	b = try! BagOfBytes(data: Data(hex: "42"))
	assert(
		b.appendingCafe().appendingDeadbeef().prependingCafe().prependingDeadbeef().hex
			== "deadbeefcafe42cafedeadbeef")

	// IMPORTANT to test all 256 values of a byte, asserting that we test
	// every single byte value that twos complement work
	(0...UInt8.max).forEach {
		let d = Data([$0])
		assert(BagOfBytes(data: d) == d)
	}

	let n = 100
	let s = Set(
		(0..<n).map {
			// probability of collision is non-existing for 16 bytes
			try! BagOfBytes(data: Data.random(byteCount: 16 + $0))
		})
	assert(s.count == n)
}

try! test()
