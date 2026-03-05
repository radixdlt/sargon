import Foundation
import SargonUniFFI

extension RadixConnectMobile {
	public static func live(os: SargonOS) -> RadixConnectMobile {
		os.radixConnectMobile()
	}
}
