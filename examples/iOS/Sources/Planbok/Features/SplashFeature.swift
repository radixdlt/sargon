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
			case hasAccounts(in: Profile)
			case noAccount
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
			Text("SPLASH")
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
						try await clock.sleep(for: .milliseconds(600))
//						if let profile = keychain.loadData(key: .)
					}
			case .delegate:
					.none
			}
		}
	}
}
