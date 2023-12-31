// MARK: Data + Hex
import Foundation
import radix_wallet_kit

protocol PublicKeyProtocol {
	func toBytes() -> Data
	func toHex() -> String
}
extension Ed25519PublicKey: PublicKeyProtocol {}
extension Secp256k1PublicKey: PublicKeyProtocol {}

extension Secp256k1PublicKey {
	func toHex() -> String {
		secp256k1PublicKeyToHex(publicKey: self)
	}
	func toBytes() -> Data {
		secp256k1PublicKeyToBytes(publicKey: self)
	}
}
extension Ed25519PublicKey {
	func toHex() -> String {
		ed25519PublicKeyToHex(publicKey: self)
	}
	func toBytes() -> Data {
		ed25519PublicKeyToBytes(publicKey: self)
	}

}

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

func testKey<K: PublicKeyProtocol>(
	type: K.Type = K.self,
	fromBytes: (Data) throws -> K,
	fromHex: (String) throws -> K,
	hex: String
) throws -> K {
	let bytes = try Data(hex: hex)
	assert(bytes.hex == hex)
	let pk0 = try fromHex(hex)
	assert(pk0.toHex() == hex)
	assert(pk0.toBytes() == bytes)
	let pk1 = try fromBytes(bytes)
	assert(pk1.toHex() == hex)
	assert(pk1.toBytes() == bytes)
	return pk0
}

func testKeysCurve25519() throws {
	let hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
	let key = try testKey(
		type: Ed25519PublicKey.self,
		fromBytes: newEd25519PublicKeyFromBytes,
		fromHex: newEd25519PublicKeyFromHex,
		hex: hex
	)
	let publicKey = PublicKey.ed25519(key: key)
	switch publicKey {
	case let .ed25519(ed25519Key): assert(ed25519Key.toHex() == hex)
	case .secp256k1: assertionFailure("Expected Ed25519 key")
	}
}

func testKeysSecp256k1() throws {
	let hex = "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
	let key = try testKey(
		type: Secp256k1PublicKey.self,
		fromBytes: newSecp256k1PublicKeyFromBytes,
		fromHex: newSecp256k1PublicKeyFromHex,
		hex: hex
	)
	let publicKey = PublicKey.secp256k1(key: key)
	switch publicKey {
	case let .secp256k1(secp256k1Key): assert(secp256k1Key.toHex() == hex)
	case .ed25519: assertionFailure("Expected secp256k1 key")
	}
}

func testKeys() throws {
	try testKeysCurve25519()
	try testKeysSecp256k1()
}

func test() throws {
	try testKeys()
}

try! test()
