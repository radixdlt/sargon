@Reducer
public struct SplashFeature {

	@Dependency(\.continuousClock) var clock
	@Dependency(\.keychain) var keychain
	
	@ObservableState
	public struct State {
		public init() {}
	}
	
	public enum Action: ViewAction {
		public enum DelegateAction {
			case walletInitialized(Wallet, hasAccount: Bool)
		}
		public enum ViewAction {
			case appear
		}
		case delegate(DelegateAction)
		case view(ViewAction)

	}
	
	@ViewAction(for: SplashFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<SplashFeature>
		public init(store: StoreOf<SplashFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
				Image("Splash", bundle: Bundle.module)
				.resizable()
				.ignoresSafeArea(edges: [.top, .bottom])
				.onAppear {
					send(.appear)
				}
		}
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.appear):
					.run { send in
						let secureStorage = Keychain.shared
						try await clock.sleep(for: .milliseconds(1200))
						if try keychain.loadData(SecureStorageKey.activeProfileId) != nil {
							let wallet = try Wallet.byLoadingProfile(secureStorage: secureStorage)
							let profile = wallet.profile()
							let hasAccount = profile.networks.first?.accounts.isEmpty == false
							await send(.delegate(.walletInitialized(wallet, hasAccount: hasAccount)))
						} else {
							await send(.delegate(.walletInitialized(
								Wallet.generateNewBDFSAndEmptyProfile(secureStorage: secureStorage),
								hasAccount: false)
							))
						}
					}
			case .delegate:
					.none
			}
		}
	}
}

extension Wallet {
	static func generateNewBDFSAndEmptyProfile(secureStorage: SecureStorage = Keychain.shared) -> Wallet {
		do {
			return try Wallet.byCreatingNewProfileAndSecretsWithEntropy(
				entropy: BagOfBytes.random(byteCount: 32),
				walletClientModel: .iphone,
				walletClientName: "Unknown iPhone",
				secureStorage: secureStorage
			)
		} catch {
			fatalError("TODO Handle errors: error \(error)")
		}
	}
}
