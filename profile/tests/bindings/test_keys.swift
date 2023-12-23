import radix_wallet_kit
import Foundation

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

        let stringBytes: [UInt8] = Array(hexString.lowercased().data(using: String.Encoding.utf8)!)

        for i in stride(from: stringBytes.startIndex, to: stringBytes.endIndex - 1, by: 2) {
            let char1 = stringBytes[i]
            let char2 = stringBytes[i + 1]

            try self.append(htoi(char1) << 4 + htoi(char2))
        }
    }
}

protocol PublicKeyProtocol {
    func toBytes()   -> Data
    func toHex()   -> String
}
extension Ed25519PublicKey: PublicKeyProtocol {}
extension Secp256k1PublicKey: PublicKeyProtocol {}

func testKey<K: PublicKeyProtocol>(
    type: K.Type = K.self, 
    fromBytes: (Data) throws -> K,
    fromHex: (String) throws -> K,
    hex: String
) throws {
    let bytes = try Data(hex: hex)
    assert(bytes.hex == hex)
    let pk0 = try fromHex(hex)
    assert(pk0.toHex() == hex)
    assert(pk0.toBytes() == bytes)
    let pk1 = try fromBytes(bytes)
    assert(pk1.toHex() == hex)
    assert(pk1.toBytes() == bytes)
}

func testKeysCurve25519() throws {
    try testKey(
        type: Ed25519PublicKey.self,
        fromBytes: Ed25519PublicKey.fromBytes,
        fromHex: Ed25519PublicKey.fromHex,
        hex: "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
    )
}

func testKeysSecp256k1() throws {
    try testKey(
        type: Secp256k1PublicKey.self, 
        fromBytes: Secp256k1PublicKey.fromBytes,
        fromHex: Secp256k1PublicKey.fromHex,
        hex: "02517b88916e7f315bb682f9926b14bc67a0e4246f8a419b986269e1a7e61fffa7"
    )
}

func testKeys() throws {
    try testKeysCurve25519()
    try testKeysSecp256k1()
}

func playground() throws {
    try testKeys()
}

try! playground()