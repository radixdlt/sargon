import ComposableArchitecture
import Foundation
import Sargon

// MARK: - IntroWhatIsShieldFeature
@Reducer
public struct IntroWhatIsShieldFeature {
	@ObservableState
	public struct State: Equatable {}

	@ObservableState
	public enum Action: ViewAction {
		public enum ViewAction {
			case continueButtonTapped
		}

		public enum DelegateAction {
			case `continue`
		}

		case view(ViewAction)
		case delegate(DelegateAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { _, action in
			switch action {
			case .view(.continueButtonTapped):
				.send(.delegate(.continue))
			case .delegate:
				.none
			}
		}
	}
}

extension IntroWhatIsShieldFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("Create a Security Shield").font(.largeTitle)

				Spacer()
				Text("Let's make sure you can always access your accounts - even if you lose your phone or buy a new one.")

				Spacer()

				Button("Create security shield") {
					send(.continueButtonTapped)
				}
			}
			.padding()
			.navigationTitle("New Shield")
		}
	}
}
