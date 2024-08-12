import SargonUniFFI

public typealias ProfileChangeEventPublisher = EventPublisher<Profile>

extension ProfileChangeEventPublisher: ProfileChangeDriver {
    public static let shared = ProfileChangeEventPublisher()

    public func handleProfileChange(changedProfile: Profile) async {
        subject.send(changedProfile)
    }
}

extension ProfileChangeDriver where Self == ProfileChangeEventPublisher {
    public static var shared: Self {
        ProfileChangeEventPublisher.shared
    }
}
