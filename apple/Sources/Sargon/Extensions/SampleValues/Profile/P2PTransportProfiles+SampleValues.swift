import SargonUniFFI

#if DEBUG
//extension P2PStunServer {
//	public static let sample: Self = newP2pStunServerSample()
//	public static let sampleOther: Self = newP2pStunServerSampleOther()
//}
//
//extension P2PTurnServer {
//	public static let sample: Self = newP2pTurnServerSample()
//	public static let sampleOther: Self = newP2pTurnServerSampleOther()
//}

extension P2PTransportProfile {
	public static let sample: Self = newP2pTransportProfileSample()
	public static let sampleOther: Self = newP2pTransportProfileSampleOther()
}

extension SavedP2PTransportProfiles {
	public static let sample: Self = newSavedP2pTransportProfilesSample()
	public static let sampleOther: Self = newSavedP2pTransportProfilesSampleOther()
}
#endif
