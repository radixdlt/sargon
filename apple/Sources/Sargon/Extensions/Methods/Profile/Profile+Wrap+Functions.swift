import Foundation
import SargonUniFFI

extension Profile {
    public init(json bytes: some DataProtocol) throws {
		self = try newProfileFromJsonBytes(json: BagOfBytes(bytes))
	}
	
	public func profileSnapshot() throws -> Data {
        try profileToJsonBytes(profile: self)
	}
}
