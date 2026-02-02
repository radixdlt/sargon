import AsyncExtensions
import Foundation
import SargonUniFFI

/// Makes it possible to type `.shared` on an initalizer/func taking
/// `some EventBusDriver` as parameter.
extension EventBusDriver where Self == EventBus {
	/// Singleton `EventBusDriver` of type `EventBus` being an `actor`  which forwards `EventNotification`s
	/// originally emitted by `SargonOS` (Rust side).
	public static var shared: Self {
		Self.shared
	}
}

// MARK: - EventBus
/// An `EventBusDriver` actor which handles incoming
/// `EventNotifications` and forwards them to any
/// subscriber of `notifications()`, being a multicasted
/// async sequence.
public final actor EventBus {
	/// A stream we multicast on.
	private let stream = AsyncThrowingPassthroughSubject<Element, any Error>()
	private let subject: Subject
	#if DEBUG
	public init() {
		subject = .init()
	}
	#else
	private init() {
		subject = .init()
	}
	#endif
}

extension EventBus {
	public typealias Element = EventNotification
	public typealias Subject = AsyncPassthroughSubject<Element>

	/// Singleton `EventBusDriver` of type `EventBus` being an `actor` which forwards `EventNotification`s
	/// originally emitted by `SargonOS` (Rust side).
	public static let shared = EventBus()

	/// A multicasted async sequence of `EventNotification` values
	/// over time, originally emitted by `SargonOS` (Rust side).
	public func notifications() -> AsyncMulticastSequence<EventBus.Subject, AsyncThrowingPassthroughSubject<EventBus.Element, any Error>> {
		subject
			.multicast(stream)
			.autoconnect()
	}
}

// MARK: EventBusDriver
extension EventBus: EventBusDriver {
	/// This method is called by `SargonOS` (Rust side) and we should
	/// "forward" the events to subscribers (Swift swide), i.e. `@SharedReader`s of profile values,
	/// which uses `notifications()` to subscribe to these
	/// values.
	public func handleEventNotification(eventNotification: EventNotification) async {
		log.debug("Handle event: \(String(describing: eventNotification.event))")
		subject.send(eventNotification)
	}
}
