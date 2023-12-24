import radix_wallet_kit

// MARK: Data + Hex
import Foundation


    // #[test]
    // fn new() {
    //     let public_key = Ed25519PublicKey::from_str(
    //         "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d",
    //     )
    //     .unwrap();

    //     assert_eq!(
    //         AccountAddress::new(public_key.into(), NetworkID::Mainnet).address(),
    //         "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm"
    //     )
    // }


func testAddress() throws {
    let key = try Ed25519PublicKey.fromHex(hex: "3e9b96a2a863f1be4658ea66aa0584d2a8847d4c0f658b20e62e3594d994d73d")
    let address = AccountAddress(publicKey: PublicKey.ed25519(key: key), networkId: .mainnet)
    assert(address.address() == "account_rdx129qdd2yp9vs8jkkn2uwn6sw0ejwmcwr3r4c3usr2hp0nau67m2kzdm")
}

func test() throws {
    try testAddress()
}

try! test()