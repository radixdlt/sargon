import Foundation
import SargonUniFFI

extension RadixConnectPassword {
    public init(jsonStringLiteral: String) throws {
        self = try newRadixConnectPasswordFromJsonString(jsonString: jsonStringLiteral)
    }
    
    public func jsonStringLiteral() -> String {
		radixConnectPasswordToJsonString(radixConnectPassword: self)
    }
}
