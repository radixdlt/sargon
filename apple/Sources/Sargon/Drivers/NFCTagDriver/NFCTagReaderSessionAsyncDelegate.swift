// import CoreNFC
// import AsyncExtensions
// import SargonUniFFI
//
// final class NFCTagReaderSessionAsyncDelegate: NSObject {
//    let onSessionDidBecomeActive: AsyncThrowingStream<Void, Error>
//    private let onSessionDidBecomeActiveContinuation: AsyncThrowingStream<Void, Error>.Continuation
//
//    let onSessionTagDetected: AsyncThrowingStream<[NFCTag], Error>
//    private let onSessionTagDetectedContinuation: AsyncThrowingStream<[NFCTag], Error>.Continuation
//
//    override init() {
//        (onSessionTagDetected, onSessionTagDetectedContinuation) = AsyncThrowingStream.makeStream()
//        (onSessionDidBecomeActive, onSessionDidBecomeActiveContinuation) = AsyncThrowingStream.makeStream()
//    }
//
// }
//
// extension NFCTag: @unchecked @retroactive Sendable {}
//
// extension NFCTagReaderSessionAsyncDelegate: NFCTagReaderSessionDelegate {
//    func tagReaderSessionDidBecomeActive(_ session: NFCTagReaderSession) {
//        print("======== Session did become active ========")
//        onSessionDidBecomeActiveContinuation.yield()
//    }
//
//    func tagReaderSession(_ session: NFCTagReaderSession, didInvalidateWithError error: any Error) {
//        let cancellationErrorCodes: [NFCReaderError.Code] = [.readerSessionInvalidationErrorSessionTimeout,
//                                            .readerSessionInvalidationErrorSessionTerminatedUnexpectedly,
//                                                             .readerSessionInvalidationErrorUserCanceled]
//
//        if let nfcError = error as? NFCReaderError {
//            let commonError = if cancellationErrorCodes.contains(nfcError.code) {
//                CommonError.NfcSessionCancelled
//            } else {
//                CommonError.NfcSessionLostTagConnection
//            }
//            print("======== Error from delegate: \(error) ========")
//            onSessionDidBecomeActiveContinuation.finish(throwing: commonError)
//            onSessionTagDetectedContinuation.finish(throwing: commonError)
//        }
//    }
//
//    func tagReaderSession(_ session: NFCTagReaderSession, didDetect tags: [NFCTag]) {
//        onSessionTagDetectedContinuation.yield(tags)
//    }
// }
//
