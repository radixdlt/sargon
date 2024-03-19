import Foundation
import Sargon
import XCTest

final class AccountAddressTests: XCTestCase {
    
    
    func testAddress() throws {
        let bech32 = "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
        
        let key = try Ed25519PublicKey(
            hex: "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d"
        )
        
        let address0 = AccountAddress(
            publicKey: PublicKey.ed25519(value: key), 
            networkID: .mainnet
        )
        XCTAssertEqual(address0.address, bech32)
        
        let address1 = try AccountAddress(validatingAddress: bech32)
        XCTAssertEqual(address1.address, bech32)
        
        XCTAssertEqual(
            accountAddressToShort(
                address: address1
            ),
            "acco...m2kzdm"
        )
        
        XCTAssertEqual(address1.networkID, .mainnet)
    }
}
