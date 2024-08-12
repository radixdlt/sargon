import SargonUniFFI

public typealias ProfileChangeEventPublisher = EventPublisher<Profile>

extension ProfileChangeEventPublisher: ProfileChangeDriver {
    public static let shared = ProfileChangeEventPublisher()

    public func handleProfileStateChange(changedProfileState: ProfileState) async {
        subject.send(changedProfileState)
    }
}

extension ProfileChangeDriver where Self == ProfileChangeEventPublisher {
    public static var shared: Self {
        ProfileChangeEventPublisher.shared
    }
}
