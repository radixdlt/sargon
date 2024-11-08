import Foundation
import SargonUniFFI

extension DappToWalletInteractionSubintentExpireAtTime {
	public var date: Date {
		.init(timeIntervalSince1970: TimeInterval(unixTimestampSeconds))
	}
}
