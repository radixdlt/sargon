import SargonUniFFI

#if DEBUG
//extension P2PIceServer {
//	public static let sample: Self = newP2PIceServerSample()
//	public static let sampleOther: Self = newP2PIceServerSampleOther()
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
