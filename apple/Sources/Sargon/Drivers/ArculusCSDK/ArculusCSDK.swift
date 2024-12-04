import SargonUniFFI
import ArculusCSDK
import Foundation

func cBufToData(buf: UnsafePointer<UInt8>, len: Int) -> Data {
    Data(bytes: buf, count: len)
}

extension Data {
    var toArray: [UInt8] {
        [UInt8](self)
    }
}

func cArrayToData<T>(val: T, len: size_t) throws -> Data {
    try withUnsafeBytes(of: val) { rawPtr -> Data in
        guard let baseAddr = rawPtr.baseAddress else {
            fatalError()
        }
        let ptr = baseAddr.assumingMemoryBound(to: UInt8.self)
        return Data(bytes: ptr, count: len)
    }
}

final class ArculusCSDKDriver: SargonUniFFI.ArculusCsdkDriver {
    func signHashPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath, hash: SargonUniFFI.Hash) throws -> SargonUniFFI.BagOfBytes {
        fatalError()
    }
    
    func getPublicKeyByPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath) throws -> SargonUniFFI.BagOfBytes {
        fatalError()
    }
    
    func buildCommand(build: (inout size_t) -> UnsafeMutablePointer<UInt8>?) throws -> Data {
        var len: size_t = 0
        guard let pointer = build(&len) else {
            fatalError()
        }
        return cBufToData(buf: pointer, len: len)
    }

    func walletInit() -> SargonUniFFI.ArculusWalletPointer {
        .init(opaquePointer: ArculusCSDK.walletInit())
    }
    
    func walletFree(wallet: SargonUniFFI.ArculusWalletPointer) {
        ArculusCSDK.walletFree(wallet: wallet.toOpaquePointer())
    }
    
    func selectWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, aid: Data) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.selectWalletRequest(walletPointer: wallet.toOpaquePointer(), aid: [UInt8](aid), len: &len)
        }
    }
    
    func selectWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, respose: Data) throws -> Int32 {
        let response = respose.toArray
        let pointer = ArculusCSDK.selectWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)

        guard let pointer else {
            fatalError()
        }

        let len = size_t(min(10, pointer.pointee.ApplicationAIDLength))
        let dat = cBufToData(buf: pointer.pointee.ApplicationAID, len: len)
        return 1
    }
    
    func createWalletSeedRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: UInt8) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.seedCreateWalletRequest(walletPointer: wallet.toOpaquePointer(), len: &len, nbrOfWords: Int(wordCount))
        }
    }
    
    func createWalletSeedResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        let response = response.toArray
        return try buildCommand { len in
            ArculusCSDK.seedCreateWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count, mnemonicSentenceLength: &len)
        }
    }
    
    func resetWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.resetWalletRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func resetWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        let response = response.toArray
        return ArculusCSDK.resetWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }
    
    func getGguidRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.getGGUIDRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func getGguidResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        let response = response.toArray
        return try buildCommand { len in
            ArculusCSDK.getGGUIDResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count, GGUIDLength: &len)
        }
    }
    
    func getFirmwareVersionRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.getFirmwareVersionRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func getFirmwareVersionResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.getFirmwareVersionResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](response), responseLength: [UInt8](response).count, versionLength: &len)
        }
    }
    
    func storeDataPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String, pinLen: UInt8) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.storeDataPINRequest(walletPointer: wallet.toOpaquePointer(), pin: pin, pinLen: Int(pin)!, len: &len)
        }
    }
    
    func storeDataPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        let response = response.toArray
        return ArculusCSDK.storeDataPINResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }
    
    func verifyPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String, pinLen: UInt8) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.verifyPINRequest(walletPointer: wallet.toOpaquePointer(), pin: pin, pinLen: Int(exactly: pinLen)!, len: &len)
        }
    }
    
    func verifyPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        let response = response.toArray
        var numberOfTries: Int = 0
        return ArculusCSDK.verifyPINResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count, nbrOfTries: &numberOfTries)
    }
    
    func initEncryptedSessionRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.initSessionRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func initEncryptedSessionResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        ArculusCSDK.initSessionResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](response), responseLength: [UInt8](response).count)
    }
    
    func getPublicKeyByPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: Data, curve: UInt16) throws -> Data {
        let path = path.toArray
        return try buildCommand { len in
            ArculusCSDK.getPublicKeyFromPathRequest(walletPointer: wallet.toOpaquePointer(), bipPath: path, bipPathLength: path.count, curve: curve, len: &len)
        }
    }
    
    func getPublicKeyByPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        let response = response.toArray
        var extendedKey = ArculusCSDK.getPublicKeyFromPathResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count).pointee
        return try cArrayToData(val: extendedKey.publicKey, len: extendedKey.pubKeyLe)
    }
    
    func signHashPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: Data, curve: UInt16, algorithm: UInt8, hash: Data) throws -> Data {
        let path = path.toArray
        let hash = hash.toArray
        return try buildCommand { len in
            ArculusCSDK.signHashRequest(walletPointer: wallet.toOpaquePointer(), bip_path: path, bip_path_length: path.count, curve: curve, algorithm: algorithm, hash: hash, hash_length: hash.count, len: &len)
        }
    }
    
    func signHashPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        let response = response.toArray
        return try buildCommand { len in
            ArculusCSDK.signHashResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count, signedHashLength: &len)
        }
    }

    func seedPhraseFromMnemonicSentence(
        wallet: SargonUniFFI.ArculusWalletPointer,
        mnemonicSentence: Data,
        mnemonicSentenceLen: UInt8,
        passphrase: Data?,
        passphraseLen: UInt8
    ) throws -> SargonUniFFI.BagOfBytes {
        try buildCommand { len in
            ArculusCSDK.seedPhraseFromMnemonicSentence(walletPointer: wallet.toOpaquePointer(), mnemonicSentence: mnemonicSentence.toArray, mnemonicSentenceLength: mnemonicSentenceLen, passphrase: passphrase?.toArray, passphraseLength: passphraseLen, seedLength: &len)
        }
    }

    func initRecoverWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: UInt8) throws -> SargonUniFFI.BagOfBytes {
        try buildCommand { len in
            ArculusCSDK.initRecoverWalletRequest(walletPointer: wallet.toOpaquePointer(), nbrOfWords: Int(wordCount), len: &len)
        }
    }

    func initRecoverWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: SargonUniFFI.BagOfBytes) throws -> Int32 {
        let response = response.toArray
        ArculusCSDK.initRecoverWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }

    func finishRecoverWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, seed: SargonUniFFI.BagOfBytes, seedLength: UInt8) throws -> SargonUniFFI.BagOfBytes {
        try buildCommand { len in
            ArculusCSDK.seedFinishRecoverWalletRequest(walletPointer: wallet.toOpaquePointer(), seed: seed.toArray, seedLength: seedLength, len: &len)
        }
    }

    func finishRecoverWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: SargonUniFFI.BagOfBytes) throws -> Int32 {
        ArculusCSDK.seedFinishRecoverWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response.toArray, responseLength: response.toArray.count)
    }
}

extension SargonUniFFI.ArculusWalletPointer {
    init(opaquePointer: OpaquePointer) {
        self.init(pointer: UInt64(bitPattern: Int64(Int(bitPattern: opaquePointer))))
    }

    func toOpaquePointer() -> OpaquePointer {
        OpaquePointer.init(bitPattern: Int(self.pointer))!
    }
}
