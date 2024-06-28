import Foundation
import SargonUniFFI

extension RadixConnectMobile {
    public static func live(sessionStorage: any RadixConnectMobileSessionStorage) -> RadixConnectMobile {
        RadixConnectMobile(networkingDriver: URLSession.shared, sessionStorage: sessionStorage)
    }
}
