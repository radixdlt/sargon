import ComposableArchitecture
import Foundation
import Sargon

// MARK: - WriteDownMnemonicFeature
@Reducer
public struct WriteDownMnemonicFeature {
	@Dependency(MnemonicClient.self) var mnemonicClient

	public init() {}

	@ObservableState
	public struct State: Equatable {
		public var mnemonic: String?
		public init() {}
	}

	public enum Action: ViewAction {
		public enum DelegateAction {
			case done
		}

		public enum ViewAction {
			case revealMnemonicButtonTapped
			case continueButtonTapped
		}

		public enum InternalAction {
			case loadedPrivateHDFactor(PrivateHierarchicalDeterministicFactorSource)
		}

		case delegate(DelegateAction)
		case view(ViewAction)
		case `internal`(InternalAction)
	}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .internal(.loadedPrivateHDFactor(privateHDFactor)):
				state.mnemonic = privateHDFactor.mnemonicWithPassphrase.mnemonic.phrase
				return .none
			case .view(.revealMnemonicButtonTapped):
				return .run { send in
					let id = try SargonOS.shared.profile().factorSources.first!.id.extract(as: FactorSourceIdFromHash.self)
					let privateHDFactor = try await mnemonicClient.loadMnemonic(id)
					await send(.internal(.loadedPrivateHDFactor(privateHDFactor)))
				} catch: { error, _ in
					fatalError("error \(error)")
				}
			case .view(.continueButtonTapped):
				return .send(.delegate(.done))
			case .delegate:
				return .none
			}
		}
	}
}

// MARK: WriteDownMnemonicFeature.View
extension WriteDownMnemonicFeature {
	@ViewAction(for: WriteDownMnemonicFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<WriteDownMnemonicFeature>

		public var body: some SwiftUI.View {
			VStack {
				Text("Write down your mnemonic on a piece of paper and put it in a safe")
					.font(.title)
				Spacer()
				if let mnemonic = store.state.mnemonic {
					Text("`\(mnemonic)`")
						.border(.yellow)
				} else {
					Button("Reveal") {
						send(.revealMnemonicButtonTapped)
					}
				}
				Spacer()
				Button("Continue") {
					send(.continueButtonTapped)
				}
			}
			.padding()
		}
	}
}
