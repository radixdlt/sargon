import CoreNFC
import AVFoundation
import SargonUniFFI

extension NFCTagReaderSession: @unchecked @retroactive Sendable {}

public actor NFCSessionClient {
    let delegate: NFCTagReaderSessionAsyncDelegate
    var session: NFCTagReaderSession?
    var isoTag: NFCISO7816Tag?

    init(delegate: NFCTagReaderSessionAsyncDelegate) {
        self.delegate = delegate
    }

    public init() {
        let delegate = NFCTagReaderSessionAsyncDelegate()
        self.init(delegate: delegate)
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

    public func sendReceiveCommandChain(commands: [Data]) async throws -> Data {
        for (index, apdu) in commands.enumerated() {
            let data = try await self.isoTag!.sendCommand(data: apdu)

            if index == commands.count - 1 {
                return data
            }
        }
        fatalError()
    }
}

extension NFCSessionClient {
    private func beginSession() async throws -> NFCISO7816Tag {
        let session = NFCTagReaderSession(pollingOption: .iso14443, delegate: self.delegate, queue: .main)!
        session.alertMessage = "Tap & hold your card to the back of your phone"
        self.session = session
        self.session!.begin()
        return try await connectTag()
    }

    private func renewSession() async throws -> NFCISO7816Tag {
        self.session!.restartPolling()
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

            try await session!.connect(to: cardTag)
            AudioServicesPlaySystemSound(SystemSoundID(kSystemSoundID_Vibrate))
            return isoTag
        }

        fatalError()
    }

    private func invalidateSession(_ isComplete: Bool = false, error: String? = nil) {
        if let err = error {
            session!.invalidate(errorMessage: err)
        } else {
            session!.invalidate()
        }
        session = nil
    }
}
