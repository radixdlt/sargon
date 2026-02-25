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

	@discardableResult
	public mutating func append(_ profile: P2PTransportProfile) -> Bool {
		guard !all.contains(where: { $0.signalingServer == profile.signalingServer }) else {
			return false
		}
		other.append(profile)
		return true
	}

	@discardableResult
	public mutating func remove(_ profile: P2PTransportProfile) -> Bool {
		let oldCount = other.count
		other.removeAll(where: { $0.signalingServer == profile.signalingServer })
		return oldCount != other.count
	}

	@discardableResult
	public mutating func changeCurrent(to profile: P2PTransportProfile) -> Bool {
		guard current.signalingServer != profile.signalingServer else {
			return false
		}

		let oldCurrent = current
		other.removeAll(where: { $0.signalingServer == profile.signalingServer })
		current = profile

		if !other.contains(where: { $0.signalingServer == oldCurrent.signalingServer }) {
			other.append(oldCurrent)
		}
		return true
	}
}

extension AppPreferences {
	public func hasP2PTransportProfile(withSignalingServerURL url: FfiUrl) -> Bool {
		appPreferencesHasP2pTransportProfileWithSignalingServerUrl(
			appPreferences: self,
			url: url
		)
	}
}
