import ComposableArchitecture
import Sargon

// MARK: - NameNewShieldFeature
@Reducer
public struct NameNewShieldFeature {
	@Dependency(ShieldClient.self) var shieldClient

	@ObservableState
	public struct State: Equatable {
		@Shared(.newShieldDraft) var newShieldDraft
		public var shieldName = ""
		public var errorMessage: String?
		var copyOf: Shield? {
			newShieldDraft.copyOf
		}

		public init() {
			if let copyOf {
				self.shieldName = "Copy of \(copyOf.metadata.displayName.value)"
			} else {
				self.shieldName = "My Shield"
			}
		}
	}

	public enum Action: ViewAction {
		public enum Delegate {
			case done
		}

		public enum InternalAction {
			case named(DisplayName)
		}

		@CasePathable
		public enum ViewAction {
			case shieldNameChanged(String)
			case continueButtonTapped
		}

		case delegate(Delegate)
		case `internal`(InternalAction)
		case view(ViewAction)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.shieldNameChanged(name)):
				state.errorMessage = nil
				state.shieldName = name
				return .none

			case .view(.continueButtonTapped):
				state.errorMessage = nil
				do {
					let displayName = try DisplayName(validating: state.shieldName)
					return .send(.internal(.named(displayName)))
				} catch {
					state.errorMessage = "Invalid DisplayName, can't be empty or too long."
					return .none
				}

			case let .internal(.named(name)):
				guard let matrixOfFactors = state.newShieldDraft.matrixOfFactors else {
					return .none
				}
				let shield = Shield(
					metadata: SecurityStructureMetadata(name: name),
					numberOfDaysUntilAutoConfirmation: state.newShieldDraft.numberOfDaysUntilAutoConfirmation,
					matrixOfFactors: matrixOfFactors
				)

				return .run { send in
					try await shieldClient.saveSecurityShield(shield)
					await send(.delegate(.done))
				}

			case .delegate:
				return .none
			}
		}
	}
}

extension NameNewShieldFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("Name Shield").font(.largeTitle)

				Spacer()
				LabeledTextField(label: "Shield Name", text: $store.shieldName.sending(\.view.shieldNameChanged))
				if let error = store.state.errorMessage {
					Text("\(error)")
						.foregroundStyle(Color.red)
						.font(.footnote)
						.fontWeight(.bold)
				}
				if let original = store.copyOf {
					let sameMatrix = original.matrixOfFactors == store.newShieldDraft.matrixOfFactors
					let copyOrEditedCopy = sameMatrix ? "Copy" : "**Edited** copy"
					Text("(\(copyOrEditedCopy) of: '\(original.metadata.displayName.value)')")
				}
				Spacer()
				Button("Continue") {
					send(.continueButtonTapped)
				}
				.buttonStyle(.borderedProminent)
			}
			.padding()
		}
	}
}
