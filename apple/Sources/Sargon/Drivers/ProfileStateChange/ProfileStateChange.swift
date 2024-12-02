import SargonUniFFI

public typealias ProfileStateChangeEventPublisher = EventPublisher<ProfileState>

// MARK: - ProfileStateChangeEventPublisher + ProfileStateChangeDriver
extension ProfileStateChangeEventPublisher: ProfileStateChangeDriver {
	public static let shared = ProfileStateChangeEventPublisher()

	public func handleProfileStateChange(changedProfileState: ProfileState) async {
		subject.send(changedProfileState)
	}
}

extension ProfileStateChangeDriver where Self == ProfileStateChangeEventPublisher {
	public static var shared: Self {
		ProfileStateChangeEventPublisher.shared
	}
}
