import Sargon
import ComposableArchitecture

@Reducer
public struct SplashFeature {

	@Dependency(\.continuousClock) var clock

	@ObservableState
	public struct State {
		public init() {}
	}

	public enum Action: ViewAction {
		public enum DelegateAction {
			case booted(hasAnyNetwork: Bool)
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
						let os = try await SargonOS.boot(bios: .shared)
						await send(.delegate(.booted(os: os, hasAnyNetwork: os.hasAnyNetwork())))
							.debounce(for: 0.8, scheduler: self.mainQueue, options: nil)

				}
			case .delegate:
				.none
			}
		}
	}
}

