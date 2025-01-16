import Foundation
import SargonUniFFI

extension AuthIntent {
	public func hash() -> AuthIntentHash {
		authIntentGetHash(authIntent: self)
	}
}
