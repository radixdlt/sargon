import Foundation
import SargonUniFFI

extension MfaFactorInstance {
	public init(jsonData: some DataProtocol) throws {
		self = try newMFAFactorInstanceFromJsonBytes(jsonBytes: Data(jsonData))
	}

	public func jsonData() -> Data {
		mFAFactorInstanceToJsonBytes(mFAFactorInstance: self)
	}
}
