import Foundation
import SargonUniFFI

extension AuthIntentHash {
	public func hash() -> Hash {
		authIntentHashGetHash(authIntentHash: self)
	}
}
