import AsyncExtensions

public final actor EventPublisher<Element: Sendable> {
	public typealias Subject = AsyncReplaySubject<Element>
	public typealias Stream = AsyncThrowingReplaySubject<Element, any Error>

	let stream = Stream(bufferSize: 1)
	let subject = Subject(bufferSize: 1)

	public func eventStream() -> AsyncMulticastSequence<Subject, Stream> {
		subject
			.multicast(stream)
			.autoconnect()
	}
}
