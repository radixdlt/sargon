@Reducer
public struct AppFeature {
	public init() {}
	
	@ObservableState
	public enum State {
		case splash(SplashFeature.State)
		case onboarding(OnboardingFeature.State)
		case main(MainFeature.State)
		public init() {
			self = .splash(.init())
		}
	}
	
	public enum Action {
		case splash(SplashFeature.Action)
		case onboarding(OnboardingFeature.Action)
		case main(MainFeature.Action)
	}
	
	public struct View: SwiftUI.View {
		public let store: StoreOf<AppFeature>
		public init(store: StoreOf<AppFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			switch store.state {
			 case .splash:
			   if let store = store.scope(state: \.splash, action: \.splash) {
				   SplashFeature.View(store: store)
			   }
			 case .onboarding:
			   if let store = store.scope(state: \.onboarding, action: \.onboarding) {
				   OnboardingFeature.View(store: store)
			   }
			 case .main:
			   if let store = store.scope(state: \.main, action: \.main) {
				   MainFeature.View(store: store)
			   }
			 }
		}
	}
}

