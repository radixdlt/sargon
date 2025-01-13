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

extension ArculusCSDKByteVector {
    func data() throws -> UnsafeMutablePointer<UInt8> {
        guard let vectorData = addr else {
            fatalError()
        }
        return vectorData
    }
}

extension ArculusCSDKAPDUSequence {
    func apdu() throws -> UnsafeMutablePointer<ArculusCSDKByteVector> {
        guard let apdu = apdu else {
            fatalError()
        }
        return apdu
    }
}

func cApduSequenceToData(buf: UnsafePointer<ArculusCSDKAPDUSequence>) throws -> [Data] {
    // Need to loop through all the ByteVectors in APDUSequence
    let apduSequence = buf.pointee
    let count = apduSequence.count
    let byteVectorArrayPtr = try apduSequence.apdu()
    var dataArray = [Data](repeating: Data(), count: Int(count))
    let byteVectorPointer = UnsafeMutablePointer<ArculusCSDKByteVector>(byteVectorArrayPtr)

    for i in 0 ..< count {
        let byteVector = byteVectorPointer[Int(i)]
        // Access the fields of each ByteVector element
        let vectorData = try byteVector.data()
        let vectorLength = byteVector.count
        let data = cBufToData(buf: vectorData, len: Int(vectorLength))
        dataArray[Int(i)] = data
    }
    return dataArray
}


final class ArculusCSDKDriver: SargonUniFFI.ArculusCsdkDriver {
    func seedPhraseFromMnemonicSentence(wallet: SargonUniFFI.ArculusWalletPointer, mnemonicSentence: SargonUniFFI.BagOfBytes, passphrase: SargonUniFFI.BagOfBytes?) throws -> SargonUniFFI.BagOfBytes {
        let sentence = try buildCommand { len in
            let sentence = ArculusCSDK.seedPhraseFromMnemonicSentence(walletPointer: wallet.toOpaquePointer(), mnemonicSentence: mnemonicSentence.toArray, mnemonicSentenceLength: mnemonicSentence.toArray.count, passphrase: passphrase?.toArray, passphraseLength: passphrase?.toArray.count ?? 0, seedLength: &len)
            return sentence
        }

        return sentence
    }

    func buildCommand(build: (inout size_t) -> UnsafeMutablePointer<UInt8>?, file: StaticString = #filePath, fun: StaticString = #function) throws -> Data {
        var len: size_t = 0
        guard let pointer = build(&len) else {
            fatalError()
        }

        let response = cBufToData(buf: pointer, len: len)

        logData(fun: fun, response: response)
        return response
    }

    func logData(fun: StaticString = #function, response: Data) {
        debugPrint("#CARD \(fun), payload: \(response.hex)")
    }

    func walletInit() -> SargonUniFFI.ArculusWalletPointer {
        .init(opaquePointer: ArculusCSDK.walletInit())
    }
    
    func walletFree(wallet: SargonUniFFI.ArculusWalletPointer) {
        ArculusCSDK.walletFree(wallet: wallet.toOpaquePointer())
    }
    
    func selectWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, aid: Data) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.selectWalletRequest(walletPointer: wallet.toOpaquePointer(), aid: aid.toArray, len: &len)
        }
    }
    
    func selectWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        logData(response: response)

        let response = response.toArray
        let pointer = ArculusCSDK.selectWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)

        guard let pointer else {
            fatalError()
        }

        let len = size_t(min(10, pointer.pointee.ApplicationAIDLength))
        let dat = cBufToData(buf: pointer.pointee.ApplicationAID, len: len)
        return 0
    }
    
    func createWalletSeedRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: Int64) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.seedCreateWalletRequest(walletPointer: wallet.toOpaquePointer(), len: &len, nbrOfWords: Int(wordCount))
        }
    }
    
    func createWalletSeedResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        logData(response: response)
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
        logData(response: response)
        let response = response.toArray
        return ArculusCSDK.resetWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }
    
    func getGguidRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.getGGUIDRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func getGguidResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        logData(response: response)
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
    
    func storeDataPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String) throws -> Data {
        return try buildCommand { len in
            ArculusCSDK.storeDataPINRequest(walletPointer: wallet.toOpaquePointer(), pin: pin, pinLen: pin.count, len: &len)
        }
    }
    
    func storeDataPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        logData(response: response)
        let response = response.toArray
        return ArculusCSDK.storeDataPINResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }
    
    func verifyPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.verifyPINRequest(walletPointer: wallet.toOpaquePointer(), pin: pin, pinLen: pin.count, len: &len)
        }
    }
    
    func verifyPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        logData(response: response)
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
        logData(response: response)
        let response = response.toArray
        return ArculusCSDK.initSessionResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }
    
    func getPublicKeyByPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: Data, curve: UInt16) throws -> Data {
        let path = path.toArray
        return try buildCommand { len in
            ArculusCSDK.getPublicKeyFromPathRequest(walletPointer: wallet.toOpaquePointer(), bipPath: path, bipPathLength: path.count, curve: curve, len: &len)
        }
    }
    
    func getPublicKeyByPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        logData(response: response)
        let response = response.toArray
        var extendedKey = ArculusCSDK.getPublicKeyFromPathResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count).pointee
        let parsed_response = try cArrayToData(val: extendedKey.publicKey, len: extendedKey.pubKeyLe)

        logData(response: parsed_response)
        return parsed_response
    }
    
    func signHashPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: Data, curve: UInt16, algorithm: UInt8, hash: Data) throws -> [Data] {
        let path = path.toArray
        let path2 = path
        let hash = hash.toArray

        var unsafePath = ArculusCSDKByteVector(count: UInt32(path2.count), addr: UnsafeMutablePointer<UInt8>.allocate(capacity: path2.count))
        unsafePath.addr.update(from: path2, count: path2.count)

        var hashData = ArculusCSDKByteVector(count: UInt32(hash.count), addr: UnsafeMutablePointer<UInt8>.allocate(capacity: hash.count))
        hashData.addr.update(from: hash, count: hash.count)
        
        var requestApduPtrPtr: UnsafeMutablePointer<UnsafeMutablePointer<ArculusCSDKAPDUSequence>?>!
        requestApduPtrPtr = UnsafeMutablePointer<UnsafeMutablePointer<ArculusCSDKAPDUSequence>?>.allocate(capacity: 1)

        let status = try ArculusCSDK.signRequest(walletPointer: wallet.toOpaquePointer(), bipPath: &unsafePath, curve: curve, algorithm: algorithm, hash: &hashData, apdus: requestApduPtrPtr)

        guard let pointer = requestApduPtrPtr.pointee else {
            fatalError()
        }
        let request = try cApduSequenceToData(buf: pointer)
        for request in request {
            logData(response: request)
        }
        return request
    }
    
    func signHashPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        logData(response: response)
        let response = response.toArray
        return try buildCommand { len in
            ArculusCSDK.signHashResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count, signedHashLength: &len)
        }
    }

    func initRecoverWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: Int64) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.initRecoverWalletRequest(walletPointer: wallet.toOpaquePointer(), nbrOfWords: Int(wordCount), len: &len)
        }
    }

    func initRecoverWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        logData(response: response)
        let response = response.toArray
        return ArculusCSDK.initRecoverWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response, responseLength: response.count)
    }

    func finishRecoverWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer, seed: SargonUniFFI.BagOfBytes) throws -> SargonUniFFI.BagOfBytes {
        try buildCommand { len in
            ArculusCSDK.seedFinishRecoverWalletRequest(walletPointer: wallet.toOpaquePointer(), seed: seed.toArray, seedLength: seed.toArray.count, len: &len)
        }
    }

    func finishRecoverWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: SargonUniFFI.BagOfBytes) throws -> Int32 {
        logData(response: response)

        return ArculusCSDK.seedFinishRecoverWalletResponse(walletPointer: wallet.toOpaquePointer(), response: response.toArray, responseLength: response.toArray.count)
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
