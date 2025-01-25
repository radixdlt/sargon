import CoreNFC
import AVFoundation
import SargonUniFFI

extension NFCTagReaderSession: @unchecked @retroactive Sendable {}

public actor NFCSessionClient {
    var delegate: NFCTagReaderSessionAsyncDelegate?
    var session: NFCTagReaderSession?
    var isoTag: NFCISO7816Tag?
    var purpose: NfcTagDriverPurpose?

    var sessionStartTime: Date = Date.now
    var sessionRenewTime: Date = Date.now

    func setIsoTag(tag: NFCISO7816Tag?) async {
        self.isoTag = tag
    }

    func setSesssionStartTime(date: Date) async {
        self.sessionStartTime = date
    }

    func setSesssionRenewTime(date: Date) async {
        self.sessionRenewTime = date
    }
}

extension NFCSessionClient: SargonUniFFI.NfcTagDriver {
    public func startSession(purpose: NfcTagDriverPurpose) async throws {
        self.purpose = purpose
        try await self.beginSession()
    }

    public func endSession(withFailure: CommonError?) async {
        self.invalidateSession(error: withFailure?.errorMessage)
        await self.setIsoTag(tag: nil)
    }

    public func sendReceive(command: Data) async throws -> Data {
        do {
            try await refreshSessionIfNeed()
            return try await self.isoTag!.sendCommand(data: command)
        } catch {
            print("======== Error from Command: \(error) ========")
            throw error
        }
    }

    public func sendReceiveCommandChain(commands: [Data]) async throws -> Data {
        try await refreshSessionIfNeed()
        for (index, apdu) in commands.enumerated() {
            let data = try await self.isoTag!.sendCommand(data: apdu)

            if index == commands.count - 1 {
                return data
            }
        }
        fatalError()
    }

    public func setMessage(message: String) async {
        switch self.purpose! {
        case let .arculus(arcPurpose):
            switch arcPurpose {
            case .identifyingCard:
                session!.alertMessage = """
        Identifying Card

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        """
            case .configuringCardMnemonic:
                session!.alertMessage = """
        Configuring the your arculus Card

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        """
            case .signTransaction(let arculusCardFactorSource):
                session!.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .signPreAuth(let arculusCardFactorSource):
                session!.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .proveOwnership(let arculusCardFactorSource):
                session!.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .derivingPublicKeys(let arculusCardFactorSource):
                session!.alertMessage = """
        Updating Factor Config

        \(message)
        
        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            }
        }
    }
}

extension NFCSessionClient {
    private func refreshSessionIfNeed() async throws {
        if self.sessionStartTime.distance(to: .now) >= 40 {
            print("========= Restarting session \(Date.now) ========== ")
            try await self.restartSession()
        } else if self.sessionRenewTime.distance(to: .now) >= 10 {
            print("========= Renewing session ========== ")
            try await self.renewSession()
        }
    }
    private func beginSession() async throws {
        let delegate = NFCTagReaderSessionAsyncDelegate()
        let session = NFCTagReaderSession(pollingOption: .iso14443, delegate: delegate, queue: .main)!
        switch purpose {
        case let .arculus(arcPurpose):
            switch arcPurpose {
            case .identifyingCard:
                session.alertMessage = """
        Identifying Card

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        """
            case .configuringCardMnemonic:
                session.alertMessage = """
        Configuring the your arculus Card

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        """
            case .signTransaction(let arculusCardFactorSource):
                session.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .signPreAuth(let arculusCardFactorSource):
                session.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .proveOwnership(let arculusCardFactorSource):
                session.alertMessage = """
        Signing Transaction

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            case .derivingPublicKeys(let arculusCardFactorSource):
                session.alertMessage = """
        Updating Factor Config

        Tap and hold this Arculus Card to your phone. This may take up to a minute.

        Card: \(arculusCardFactorSource.hint.label)
        """
            }
        default:
            break
        }

        self.session = session
        self.delegate = delegate
        session.begin()
        print("======== Session begin called \(Date.now)========")
        await self.setSesssionStartTime(date: .now)
        await self.setSesssionRenewTime(date: .now)
        print("======== Session begin connecting tag ========")
        let tag = try await connectTag()
        await self.setIsoTag(tag: tag)
    }

    private func renewSession() async throws {
        print("NFCSession renewing")
        self.session!.restartPolling()
        let tag = try await connectTag()
        await self.setIsoTag(tag: tag)
        await self.setSesssionRenewTime(date: .now)

    }

    private func restartSession() async throws {
        self.invalidateSession(true)
            try await ContinuousClock().sleep(for: .seconds(5))
            try await self.beginSession()
    }

    private func connectTag() async throws -> NFCISO7816Tag {
        for try await tags in self.delegate!.onSessionTagDetected.prefix(1) {
            let tag = tags.first { tag in
                if case .iso7816 = tag {
                    return true
                } else {
                    return false
                }
            }

            guard let cardTag = tag, case let .iso7816(isoTag) = tag else {
                await self.invalidateSession(error: "Unknown Arculus Card")
                throw CommonError.NfcSessionUnknownTag
            }

            // TODO: Check against pre-configured ids in the info.plist
            guard isoTag.initialSelectedAID == "415243554C5553010157" else {
                struct UnknownCardError: Error {}
                self.invalidateSession(error: "Unknown Arculus Card")
                throw CommonError.NfcSessionUnknownTag
            }

                try await self.session!.connect(to: cardTag)
                AudioServicesPlaySystemSound(SystemSoundID(kSystemSoundID_Vibrate))
                return isoTag

        }
        throw CancellationError()
    }

    private func invalidateSession(_ isComplete: Bool = false, error: String? = nil) {
        if let err = error {
            session!.invalidate(errorMessage: err)
        } else {
            session!.invalidate()
        }
        session = nil
        delegate = nil
        isoTag = nil
    }
}
