import Sargon
import ComposableArchitecture

@Reducer
public struct AppFeature {
	
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
	
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			
			case let .splash(.delegate(.booted(hasAnyNetwork))):
				if hasAnyNetwork {
					state = .main(MainFeature.State())
				} else {
					state = .onboarding(OnboardingFeature.State())
				}
				return .none
			
			case .onboarding(.delegate(.done)):
				state = .main(MainFeature.State())
				return .none
				
			case .main(.delegate(.deletedWallet)):
				state = .onboarding(OnboardingFeature.State())
				return .none
			
			default:
				return .none
			}
		}
		.ifCaseLet(\.splash, action: \.splash) {
			SplashFeature()
		}
		.ifCaseLet(\.onboarding, action: \.onboarding) {
			OnboardingFeature()
		}
		.ifCaseLet(\.main, action: \.main) {
			MainFeature()
		}
	}
}

