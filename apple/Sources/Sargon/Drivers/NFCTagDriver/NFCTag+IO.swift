// import CoreNFC
//
// extension NFCISO7816Tag {
//    public func sendCommand(data: Data, file: StaticString = #filePath, fun: StaticString = #function) async throws -> Data {
//        guard let command = NFCISO7816APDU(data: data) else {
//            throw NFCReaderError(.readerErrorInvalidParameterLength)
//        }
//
//        let (response, statusBytesSW1, statusBytesSW2) = try await sendCommand(apdu: command)
//        let result = response + Data([statusBytesSW1]) + Data([statusBytesSW2])
//
//        print("# NFC request response for \(fun), request: \(data.hex), response: \(result.hex)")
//        return result
//    }
//
//    func sendCommandChain(_ apdus: [Data]) async throws -> Data {
//        for (index, apdu) in apdus.enumerated() {
//            let data = try await sendCommand(data: apdu)
//
//            if index == apdus.count - 1 {
//                return data
//            }
//        }
//
//        fatalError()
//    }
// }
