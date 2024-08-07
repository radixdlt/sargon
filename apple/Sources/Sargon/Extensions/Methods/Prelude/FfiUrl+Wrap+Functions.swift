import Foundation
import SargonUniFFI

#if DEBUG
import XCTestDynamicOverlay
#endif // DEBUG

extension FfiUrl {
	public var url: URL {
		ffiUrlGetUrl(ffiUrl: self)
	}
}
