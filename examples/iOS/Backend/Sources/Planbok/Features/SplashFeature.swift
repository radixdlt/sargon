import Sargon
import ComposableArchitecture

@Reducer
public struct SplashFeature {

	@Dependency(\.continuousClock) var clock
	@ObservableState
	public struct State {
		let isEmulatingFreshInstall: Bool
		public init(isEmulatingFreshInstall: Bool = false) {
			self.isEmulatingFreshInstall = isEmulatingFreshInstall
		}
	}

	public enum Action: ViewAction, Sendable {
		public enum DelegateAction: Sendable {
			case booted(hasAnyAccount: Bool)
		}
		public enum ViewAction: Sendable {
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

	@Dependency(\.mainQueue) var mainQueue
	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce {
			state,
			action in
			switch action {
				
			case .view(.appear):
				struct SplashID: Hashable { }
				return .run { [isEmulatingFreshInstall = state.isEmulatingFreshInstall] send in
					
					let os = try await SargonOS.createdSharedBootingWith(
						bios: BIOS.shared,
						isEmulatingFreshInstall: isEmulatingFreshInstall
					)
					await send(
						.delegate(.booted(hasAnyAccount: !os.accountsForDisplayOnCurrentNetwork().isEmpty))
					)
				}
				.debounce(id: SplashID(), for: 0.8, scheduler: mainQueue)
			
			case .delegate:
				return .none
			}
		}
	}
}

