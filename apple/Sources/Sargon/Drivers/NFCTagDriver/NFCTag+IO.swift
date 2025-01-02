import CoreNFC

extension NFCISO7816Tag {
    public func sendCommand(data: Data, file: StaticString = #filePath, fun: StaticString = #function) async throws -> Data {
        guard let command = NFCISO7816APDU(data: data) else {
            throw NFCReaderError(.readerErrorInvalidParameterLength)
        }

        let (response, statusBytesSW1, statusBytesSW2) = try await sendCommand(apdu: command)
        let result = response + Data([statusBytesSW1]) + Data([statusBytesSW2])

        print("# NFC request response for \(fun), request: \(data.hex), response: \(result.hex)")
        return result
    }

    func sendCommandChain(_ apdus: [Data]) async throws -> Data {
        for (index, apdu) in apdus.enumerated() {
            let data = try await sendCommand(data: apdu)

            if index == apdus.count - 1 {
                return data
            }
        }

        throw CardReaderError.operationFailed
    }

    private func validateStatusBytes(sw1: UInt8, sw2: UInt8) throws {
        if sw1 != 0x90 && sw2 != 0x00 {
            throw CardReaderError.operationFailed
        }
    }
}

enum CardReaderError: LocalizedError {
    case invalidParameter
    case invalidCard
    case verifyPinFailed(Int)
    case operationFailed
    case connectionLost

    var errorDescription: String? {
        switch self {
        case .invalidParameter:
            return "An invalid parameter was supplied"
        case .operationFailed:
            return "The operation couldn't be completed"
        case .invalidCard:
            return "Wrong Arculus card"
        case .connectionLost:
            return "Connection lost"
        case let .verifyPinFailed(tries):
            return "PIN-code doesn’t match. \(tries) \(tries == 1 ? "try" : "tries") remaining before lockout"
        }
    }
}
