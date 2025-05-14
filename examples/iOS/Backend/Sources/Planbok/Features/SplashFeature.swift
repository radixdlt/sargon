import ComposableArchitecture
@testable import Sargon

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
			case booted(hasAnyAccountOnAnyNetwork: Bool)
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
			Text("Swift Sargon")
				.font(.largeTitle)
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
				struct SplashID: Hashable {}
				return .run { [isEmulatingFreshInstall = state.isEmulatingFreshInstall] send in
					let os = try await SargonOS._creatingShared(
						bootingWith: BIOS.shared,
						isEmulatingFreshInstall: isEmulatingFreshInstall
					)
					let hasAnyAccountOnAnyNetwork = (try? os.hasAnyAccountOnAnyNetwork()) ?? false
					await send(
						.delegate(.booted(
							hasAnyAccountOnAnyNetwork: hasAnyAccountOnAnyNetwork
						))
					)
				}
				.debounce(id: SplashID(), for: 0.8, scheduler: mainQueue)

			case .delegate:
				return .none
			}
		}
	}
}
