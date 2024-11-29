import ComposableArchitecture
import Foundation
import Sargon
import UniformTypeIdentifiers

// MARK: - ImportProfileFlowFeature
@Reducer
public struct ImportProfileFlowFeature {
	@Dependency(ProfileClient.self) var profileClient

	@Reducer(state: .equatable)
	public enum Path {
		case inputDecryptionPassword(InputDecryptionPasswordFeature)
	}

	@ObservableState
	public struct State: Equatable {
		public var path = StackState<Path.State>()
		public var selectFile: SelectFileFeature.State

		public init() {
			self.selectFile = SelectFileFeature.State()
		}
	}

	public enum Action {
		public enum DelegateAction {
			case imported
			case failed
		}

		case path(StackAction<Path.State, Path.Action>)

		case selectFile(SelectFileFeature.Action)
		case delegate(DelegateAction)
	}

	public init() {}

	public var body: some ReducerOf<Self> {
		Scope(state: \.selectFile, action: \.selectFile) {
			SelectFileFeature()
		}

		Reduce { state, action in
			switch action {
			case let .selectFile(.delegate(.analyzedContentsOfFile(contents: data, analysis: profileFileContents))):
				switch profileFileContents {
				case .notProfile:
					return .send(.delegate(.failed))

				case .encryptedProfile:
					state.path.append(.inputDecryptionPassword(InputDecryptionPasswordFeature.State(encryptedProfile: data)))
					return .none

				case let .plaintextProfile(plaintextProfile):
					return importProfile(&state, plaintextProfile)
				}

			case let .path(pathAction):
				switch pathAction {
				case let .element(
					id: _,
					action: .inputDecryptionPassword(.delegate(.inputtedPassword(encryptedProfile, decryptionPassword)))
				):
					do {
						let decrypted = try profileClient.decryptEncryptedProfile(encryptedProfile, decryptionPassword)
						state.path = .init()
						return importProfile(&state, decrypted)
					} catch {
						log.error("Failed to decrypt encrypted profile, error: \(error)")
					}
				case .element(id: _, action: _):
					return .none
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				}
				return .none

			case .selectFile:
				return .none

			case .delegate:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
	}

	func importProfile(_ state: inout State, _ profile: Profile) -> Effect<Action> {
		.run { send in
			try await profileClient.importProfile(profile)
			await send(.delegate(.imported))
		}
	}
}

// MARK: ImportProfileFlowFeature.View
extension ImportProfileFlowFeature {
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<ImportProfileFlowFeature>
		public init(store: StoreOf<ImportProfileFlowFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				SelectFileFeature.View(
					store: store.scope(state: \.selectFile, action: \.selectFile)
				)
			} destination: { store in
				switch store.case {
				case let .inputDecryptionPassword(store):
					InputDecryptionPasswordFeature.View(store: store)
				}
			}
		}
	}
}
