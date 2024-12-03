import SargonUniFFI
import ArculusCSDK
import Foundation

func cBufToData(buf: UnsafePointer<UInt8>, len: Int) -> Data {
    Data(bytes: buf, count: len)
}

final class ArculusCSDKDriver: SargonUniFFI.ArculusCsdkDriver {
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
        let pointer = ArculusCSDK.selectWalletResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](respose), responseLength: [UInt8](respose).count)

        guard let pointer else {
            fatalError()
        }

        let len = size_t(min(10, pointer.pointee.ApplicationAIDLength))
        let dat = cBufToData(buf: pointer.pointee.ApplicationAID, len: len)
        return 1
    }
    
    func createWalletSeedRequest(wallet: SargonUniFFI.ArculusWalletPointer, wordCount: UInt8) throws -> Data {
        fatalError()
    }
    
    func createWalletSeedResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        fatalError()
    }
    
    func resetWalletRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        fatalError()
    }
    
    func resetWalletResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        fatalError()
    }
    
    func getGguidRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        fatalError()
    }
    
    func getGguidResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        fatalError()
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
        fatalError()
    }
    
    func storeDataPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        fatalError()
    }
    
    func verifyPinRequest(wallet: SargonUniFFI.ArculusWalletPointer, pin: String, pinLen: UInt8) throws -> Data {
        fatalError()
    }
    
    func verifyPinResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        fatalError()
    }
    
    func initEncryptedSessionRequest(wallet: SargonUniFFI.ArculusWalletPointer) throws -> Data {
        try buildCommand { len in
            ArculusCSDK.initSessionRequest(walletPointer: wallet.toOpaquePointer(), len: &len)
        }
    }
    
    func initEncryptedSessionResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Int32 {
        ArculusCSDK.initSessionResponse(walletPointer: wallet.toOpaquePointer(), response: [UInt8](response), responseLength: [UInt8](response).count)
    }
    
    func getPublicKeyByPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath) throws -> Data {
        fatalError()
    }
    
    func getPublicKeyByPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        fatalError()
    }
    
    func signHashPathRequest(wallet: SargonUniFFI.ArculusWalletPointer, path: SargonUniFFI.DerivationPath, hash: SargonUniFFI.Hash) throws -> Data {
        fatalError()
    }
    
    func signHashPathResponse(wallet: SargonUniFFI.ArculusWalletPointer, response: Data) throws -> Data {
        fatalError()
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
