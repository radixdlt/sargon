import Foundation
import SargonUniFFI

public typealias P2PStunServer = P2pStunServer
public typealias P2PTurnServer = P2pTurnServer
public typealias P2PTransportProfile = P2pTransportProfile
public typealias SavedP2PTransportProfiles = SavedP2pTransportProfiles

//extension P2PStunServer: SargonModel {}
//extension P2PTurnServer: SargonModel {}
extension P2PTransportProfile: SargonModel {}
extension SavedP2PTransportProfiles: SargonModel {}
