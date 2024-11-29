import Foundation
import SargonUniFFI

extension Instant {
	public var date: Date {
		.init(timeIntervalSince1970: TimeInterval(secondsSinceUnixEpoch))
	}
}
