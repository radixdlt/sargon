import Foundation
import SargonUniFFI
import AsyncExtensions

// Makes it possible to type `.shared` on an initalizer/func taking
// `some EventBusDriver` as parameter.
extension ProfileChangeDriver where Self == ProfileChangeBus {

    public static var shared: Self { Self.shared }
}

/// An `EventBusDriver` actor which handles incoming
/// `EventNotifications` and forwards them to any
/// subscriber of `notifications()`, being a multicasted
/// async sequence.
public final actor ProfileChangeBus {
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

extension ProfileChangeBus {

    public typealias Element = Profile
    public typealias Subject = AsyncPassthroughSubject<Element>

    public static let shared = ProfileChangeBus()

    /// A multicasted async sequence of `EventNotification` values
    /// over time, originally emitted by `SargonOS` (Rust side).
    public func profile_change_stream() -> AsyncMulticastSequence<Subject, AsyncThrowingPassthroughSubject<Element, any Error>> {
        subject
         .multicast(stream)
         .autoconnect()
    }
}

extension ProfileChangeBus: ProfileChangeDriver {
    public func handleProfileChange(changedProfile: Profile) async {
        subject.send(changedProfile)
    }
}
