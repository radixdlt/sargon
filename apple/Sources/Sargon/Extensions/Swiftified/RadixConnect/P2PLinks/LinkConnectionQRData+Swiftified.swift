import Foundation
import SargonUniFFI

public typealias LinkConnectionQRData = LinkConnectionQrData

// MARK: - LinkConnectionQrData + SargonModel
extension LinkConnectionQrData: SargonModel {}

// MARK: - LinkConnectionQrData + SargonObjectCodable
extension LinkConnectionQrData: SargonObjectCodable {}
