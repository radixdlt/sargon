import Sargon
import UniformTypeIdentifiers
import Foundation
import ComposableArchitecture

extension UTType {
	// FIXME: should we declare our own file format? For now we use require `.json` file extension.
	public static let profile: Self = .json
}


extension String {
	static let profileFileEncryptedPart = "encrypted"
	private static let filenameProfileBase = "radix_wallet_backup_file"
	static let filenameProfileNotEncrypted: Self = "\(filenameProfileBase).plaintext.json"
	static let filenameProfileEncrypted: Self = "\(filenameProfileBase).\(profileFileEncryptedPart).json"
}

struct LackedPermissionToAccessSecurityScopedResource: Error {}

@Reducer
public struct SelectFileFeature {
	
	@CasePathable
	public enum Action: ViewAction {
		public enum DelegateAction {
			case analyzedContentsOfFile(contents: Data, analysis: ProfileFileContents)
		}
		
		@CasePathable
		public enum ViewAction {
			case openFileButtonTapped
			case profileImportResult(Result<URL, NSError>)
			case isPresentingFileImporterChanged(Bool)
		}
		case delegate(DelegateAction)
		case view(ViewAction)
	}
	
	@ObservableState
	public struct State: Equatable {
		public var isPresentingFileImporter = false
		public init() {}
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.openFileButtonTapped):
				state.isPresentingFileImporter = true
				return .none
				
			case let .view(.profileImportResult(.failure(error))):
				log.error("Failed to read file, error: \(error)")
				return .none
				
			case let .view(.profileImportResult(.success(profileURL))):
				do {
					guard profileURL.startAccessingSecurityScopedResource() else {
						throw LackedPermissionToAccessSecurityScopedResource()
					}
					defer { profileURL.stopAccessingSecurityScopedResource() }
					let data = try Data(contentsOf: profileURL)
					
					let analyzed = Profile.analyzeContents(data: data)
					return .send(.delegate(.analyzedContentsOfFile(contents: data, analysis: analyzed)))
					
				} catch {
					log.error("Failed to import profile, error: \(error)")
				}
				return .none
				
				
			case let .view(.isPresentingFileImporterChanged(isPresentingFileImporter)):
				state.isPresentingFileImporter = isPresentingFileImporter
				return .none
				
			case .delegate(_):
				return .none
			}
		}
	}
}

extension SelectFileFeature {
	
	@ViewAction(for: SelectFileFeature.self)
	public struct View: SwiftUI.View {
		
		@Bindable public var store: StoreOf<SelectFileFeature>
		
		public init(store: StoreOf<SelectFileFeature>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			VStack {
				Text("Select file")
				
				Button("Open file selector") {
					send(.openFileButtonTapped)
				}
			}
			.fileImporter(
				isPresented: $store.isPresentingFileImporter.sending(\.view.isPresentingFileImporterChanged),
				allowedContentTypes: [.profile],
				onCompletion: { send(.profileImportResult($0.mapError { $0 as NSError })) }
			)
			.navigationTitle("Open Profile file")
		}
	}
	
}
