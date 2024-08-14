import AsyncExtensions

public final actor EventPublisher<Element: Sendable> {
    public typealias Subject = AsyncPassthroughSubject<Element>
    public typealias Stream = AsyncThrowingPassthroughSubject<Element, any Error>

    let stream = Stream()
    let subject = Subject()

    public func eventStream() -> AsyncMulticastSequence<Subject, Stream> {
        subject
         .multicast(stream)
         .autoconnect()
    }
}
