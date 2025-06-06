import ComposableArchitecture
import Foundation
import Sargon

// MARK: - NewHWFactorSourceFeature
@Reducer
public struct NewHWFactorSourceFeature {
	@Dependency(FactorSourcesClient.self) var factorSourcesClient

	@ObservableState
	public struct State: Equatable {
		public let kind: FactorSourceKind
		public var inputMnemonic: InputMnemonicFeature.State
		public init(kind: FactorSourceKind) {
			self.kind = kind
			self.inputMnemonic = .init()
		}
	}

	@CasePathable
	public enum Action {
		case `internal`(InternalAction)
		case delegate(DelegateAction)
		@CasePathable
		public enum InternalAction {
			case inputMnemonic(InputMnemonicFeature.Action)
		}

		@CasePathable
		public enum DelegateAction {
			case createdAndSavedNewFactorSource
		}
	}

	public var body: some ReducerOf<Self> {
		Scope(state: \.inputMnemonic, action: \.internal.inputMnemonic) {
			InputMnemonicFeature()
		}

		Reduce { state, action in
			switch action {
			case let .internal(.inputMnemonic(.delegate(.confirmed(mnemonicWithPassphrase)))):
				.run { [kind = state.kind] send in
					let factorSource = try await factorSourcesClient.createHWFactorSource(mnemonicWithPassphrase, kind)
					try await factorSourcesClient.addFactorSource(factorSource)
					await send(.delegate(.createdAndSavedNewFactorSource))
				}

			case .delegate:
				.none

			case .internal:
				.none
			}
		}
	}
}

extension NewHWFactorSourceFeature {
	public typealias HostingFeature = Self
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("New \(store.state.kind) Factor")
				InputMnemonicFeature.View(store: store.scope(state: \.inputMnemonic, action: \.internal.inputMnemonic))
			}
		}
	}
}
