import Foundation
import SargonUniFFI

extension DataProtocol {
	public func hash() -> Hash {
		SargonUniFFI.hash(data: Data(self))
	}
}

extension Hash {
	public func hash() -> Self {
		data.hash()
	}
}

extension Hash {
    public var data: Data {
		bytes.data
    }
    
	public var bytes: Exactly32Bytes {
		hashGetBytes(hash: self)
	}
}
