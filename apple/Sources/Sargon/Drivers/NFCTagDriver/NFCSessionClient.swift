import CoreNFC
import AVFoundation
import SargonUniFFI

extension NFCTagReaderSession: @unchecked @retroactive Sendable {}

public actor NFCSessionClient {
    let delegate: NFCTagReaderSessionAsyncDelegate
    let session: NFCTagReaderSession
    var isoTag: NFCISO7816Tag?

    init(delegate: NFCTagReaderSessionAsyncDelegate, session: NFCTagReaderSession) {
        self.delegate = delegate
        self.session = session
    }

    public init() {
        let delegate = NFCTagReaderSessionAsyncDelegate()
        let session = NFCTagReaderSession(pollingOption: .iso14443, delegate: delegate, queue: .main)!
        session.alertMessage = "Tap & hold your card to the back of your phone"
        self.init(delegate: delegate, session: session)
    }

    func setIsoTag(tag: NFCISO7816Tag?) async {
        self.isoTag = tag
    }
}

extension NFCSessionClient: SargonUniFFI.NfcTagDriver {
    public func startSession() async throws {
        let tag = try await self.beginSession()
        await self.setIsoTag(tag: tag)
    }

    public func endSession() async {
        self.invalidateSession()
        await self.setIsoTag(tag: nil)
    }

    public func sendReceive(command: Data) async throws -> Data {
        try await self.isoTag!.sendCommand(data: command)
    }
}

//extension NFCSessionClient {
//    public func getCardUUID() async throws -> Data {
//        try await executeOperation(operation: cardService.getGGUID)
//    }
//
//    public func getFirmwareVersion() async throws -> String {
//        try await executeOperation(operation: cardService.getFirmwareVersion)
//    }
//
//    public func resetWallet() async throws {
//        try await executeOperation(operation: cardService.resetWallet)
//    }
//
//    public func startCreateWalletSeed(pin: String, wordCount: Int) async throws -> [String] {
//        try await executeOperation { tag in
//            try await cardService.startCreateWalletSeed(tag: tag, pin: pin, wordCount: 24)
//        }
//    }
//
//    public func updatePin(oldPin: String, newPin: String) async throws {
//        try await executeOperation { tag in
//            try await cardService.updatePin(tag: tag, oldPin: oldPin, newPin: newPin)
//        }
//    }
//
//    public func verifyPin(pin: String) async throws -> (Bool, Int) {
//        try await executeOperation { tag in
//            try await cardService.verifyPin(tag: tag, pin: pin)
//        }
//    }
//
//    public func storePin(pin: String) async throws {
//        try await executeOperation { tag in
//            try await cardService.storePin(tag: tag, pin: pin)
//        }
//    }
//
//    public func getPublicKeyByPath(path: String, curve: CardCurve) async throws -> PublicKey {
//        try await executeOperation { tag in
//            try await cardService.getPubKeyByPath(tag: tag, path: path, curve: curve)
//        }
//    }
//
//    public func recoverWallet(pin: String, words: [String]) async throws {
//        try await executeOperation { tag in
//            try await cardService.restoreWalletSeed(tag: tag, pin: pin, words: words)
//        }
//    }
//
//    public func createWalletSeed(pin: String, wordCount: Int) async throws -> [String] {
//        try await executeOperation { tag in
//            let words = try await cardService.startCreateWalletSeed(tag: tag, pin: pin, wordCount: wordCount)
//            let seed = try cardService.seedFromWords(words: words)
//            try await cardService.finishCreateWalletSeed(tag: tag, pin: pin, seed: seed)
//            return words
//        }
//    }
//
//    public func signHashPaths(pin: String, hash: Data, path: String) async throws -> (Data, PublicKey) {
//        try await executeOperation { tag in
//            try await cardService.signHashPath(tag: tag, pin: pin, path: path, curve: .ed25519Curve, algorithm: .eddsa, hash: hash)
//        }
//    }
//}

extension NFCSessionClient {
    private func beginSession() async throws -> NFCISO7816Tag {
        self.session.begin()
        return try await connectTag()
    }

    private func renewSession() async throws -> NFCISO7816Tag {
        self.session.restartPolling()
        return try await connectTag()
    }

    private func connectTag() async throws -> NFCISO7816Tag {
        for try await tags in delegate.onSessionTagDetected.prefix(1) {
            let tag = tags.first { tag in
                if case .iso7816 = tag {
                    return true
                } else {
                    return false
                }
            }

            guard let cardTag = tag, case let .iso7816(isoTag) = tag else {
                self.invalidateSession(error: "Connection Lost.")
                fatalError()
            }

            try await session.connect(to: cardTag)
            AudioServicesPlaySystemSound(SystemSoundID(kSystemSoundID_Vibrate))
            return isoTag
        }

        fatalError()
    }

    private func invalidateSession(_ isComplete: Bool = false, error: String? = nil) {
        if let err = error {
            session.invalidate(errorMessage: err)
        } else {
            session.invalidate()
        }
    }

    private func executeOperation<Response: Sendable>(operation: (NFCISO7816Tag) async throws -> Response) async throws -> Response {
        let tag = try await beginSession()
        do {
            let response = try await operation(tag)
            session.invalidate()
            return response
        } catch {
            session.invalidate(errorMessage: error.localizedDescription)
            throw error
        }
    }
}
