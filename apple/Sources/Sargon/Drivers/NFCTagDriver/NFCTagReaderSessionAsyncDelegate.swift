import CoreNFC

final class NFCTagReaderSessionAsyncDelegate: NSObject {
    let onSessionDidBecomeActive: AsyncStream<Void>
    private let onSessionDidBecomeActiveContinuation: AsyncStream<Void>.Continuation

    let onSessionDidInvalidateError: AsyncStream<NFCReaderError>
    private let onSessionDidInvalidateErrorContinuation: AsyncStream<NFCReaderError>.Continuation

    let onSessionTagDetected: AsyncStream<[NFCTag]>
    private let onSessionTagDetectedContinuation: AsyncStream<[NFCTag]>.Continuation

    override init() {
        (onSessionTagDetected, onSessionTagDetectedContinuation) = AsyncStream.makeStream()
        (onSessionDidBecomeActive, onSessionDidBecomeActiveContinuation) = AsyncStream.makeStream()
        (onSessionDidInvalidateError, onSessionDidInvalidateErrorContinuation) = AsyncStream.makeStream()
    }

}

extension NFCTag: @unchecked @retroactive Sendable {}

extension NFCTagReaderSessionAsyncDelegate: NFCTagReaderSessionDelegate {
    func tagReaderSessionDidBecomeActive(_ session: NFCTagReaderSession) {
        onSessionDidBecomeActiveContinuation.yield()
    }
    
    func tagReaderSession(_ session: NFCTagReaderSession, didInvalidateWithError error: any Error) {
        if let nfcError = error as? NFCReaderError {
            onSessionDidInvalidateErrorContinuation.yield(nfcError)
        }
    }
    
    func tagReaderSession(_ session: NFCTagReaderSession, didDetect tags: [NFCTag]) {
        onSessionTagDetectedContinuation.yield(tags)
    }
}

