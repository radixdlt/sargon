import profileFFI

func playground() throws {
    let hex = "ec172b93ad5e563bf4932c70e1245034c35467ef2efd4d64ebf819683467e2bf"
    let ed25519PublicKey = try Ed25519PublicKey.fromHex(hex: hex)
    assert(ed25519PublicKey.toHex() == hex)
}

try! playground()