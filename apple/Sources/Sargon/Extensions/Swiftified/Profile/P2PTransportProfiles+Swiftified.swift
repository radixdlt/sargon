import Foundation
import SargonUniFFI

public typealias P2PStunServer = P2pStunServer
public typealias P2PTurnServer = P2pTurnServer
public typealias P2PTransportProfile = P2pTransportProfile
public typealias SavedP2PTransportProfiles = SavedP2pTransportProfiles

// MARK: - P2PTransportProfile + SargonModel
// extension P2PStunServer: SargonModel {}
// extension P2PTurnServer: SargonModel {}
extension P2PTransportProfile: SargonModel {}

// MARK: - SavedP2PTransportProfiles + SargonModel
extension SavedP2PTransportProfiles: SargonModel {}
