import SargonUniFFI
import ArculusCSDK

func cBufToData(buf: UnsafePointer<UInt8>, len: Int) -> Data {
    Data(bytes: buf, count: len)
}

final class ArculusCSDKDriver: SargonUniFFI.ArculusCsdkDriver {
    func buildCommand(build: (inout size_t) -> UnsafeMutablePointer<UInt8>?) throws -> Data {
        var len: size_t = 0
        guard let pointer = build(&len) else {
            throw ArculusWalletError.invalidPointer
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
        buildCommand { len in
            ArculusCSDK.selectWalletRequest(walletPointer: wallet.toOpaquePointer(), aid: [UInt8](aid), len: &len)
        }

    }
    
    func selectWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, respose: Data) throws -> Int32 {
        buildCommand { len in
            ArculusCSDK.selectWalletResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](respose), responseLength: [UInt8](respose).count)
        }
    }
    
    func createWalletSeedRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: UInt8) throws -> Data {
        buildCommand { len in
            ArculusCSDK.seedCreateWalletRequest(walletPointer: wallet.toOpaquePointer(), len: &len, nbrOfWords: wordCount)
        }
    }
    
    func createWalletSeedResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        buildCommand { len in
            ArculusCSDK.seedCreateWalletResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](response), responseLength: [UInt8](response).count, mnemonicSentenceLength: &len)
        }
    }
    
    func resetWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        buildCommand { len in
            ArculusCSDK.resetWalletRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func resetWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        buildCommand { len in
            ArculusCSDK.resetWalletResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](response), responseLength: [UInt8](response).count)
        }
    }
    
    func getGguidRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {

    }
    
    func getGguidResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {

    }
    
    func getFirmwareVersionRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {

    }
    
    func getFirmwareVersionResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {

    }
    
    func storeDataPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String, pinLen: UInt8) throws -> Data {

    }
    
    func storeDataPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {

    }
    
    func verifyPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String, pinLen: UInt8) throws -> Data {

    }
    
    func verifyPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {

    }
    
    func initEncryptedSessionRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {

    }
    
    func initEncryptedSessionResponse(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Int32 {

    }
    
    func getPublicKeyByPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath) throws -> Data {

    }
    
    func getPublicKeyByPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {

    }
    
    func signHashPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath, hash: SargonUniFFI.Hash) throws -> Data {

    }
    
    func signHashPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {

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
