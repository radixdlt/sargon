import Foundation
import SargonUniFFI

extension SavedP2PTransportProfiles {
	public init(current: P2PTransportProfile) {
		self = newSavedP2pTransportProfiles(current: current)
	}

	public var all: [P2PTransportProfile] {
		savedP2pTransportProfilesGetAllElements(profiles: self)
	}

	public static let `default`: Self = newSavedP2pTransportProfilesDefault()
}

extension AppPreferences {
	public func hasP2PTransportProfile(withSignalingServer signalingServer: String) -> Bool {
		appPreferencesHasP2pTransportProfileWithSignalingServer(
			appPreferences: self,
			signalingServer: signalingServer
		)
	}
}
