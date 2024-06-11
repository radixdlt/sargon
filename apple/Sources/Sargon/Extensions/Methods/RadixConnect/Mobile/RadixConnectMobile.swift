import Foundation
import SargonUniFFI

extension RadixConnectMobile {
    public static func live(sessionStorage: any SessionStorage) -> RadixConnectMobile {
        RadixConnectMobile(networkAntenna: URLSession.shared, sessionStorage: sessionStorage)
    }
}
