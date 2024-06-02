import SwiftUI
import Sargon
import ComposableArchitecture

extension SecurityNotProductionReadyQuestion: Identifiable {
	public typealias ID = UInt16
}
public struct AnswersToQuestions: Hashable, Sendable {
	public let questions: IdentifiedArrayOf<SecurityNotProductionReadyQuestion>
	public var answers: [SecurityNotProductionReadyQuestion.ID: String] = [:]
}

extension PersistenceKey where Self == InMemoryKey<AnswersToQuestions> {
	static var answersToSecurityQuestions: Self {
		.inMemory("answersToSecurityQuestions")
	}
}

@Reducer
public struct SelectQuestionsFeature {
	public struct State: Equatable {}
	public enum Action {
		case delegate(DelegateAction)
		public enum DelegateAction {
			case done
		}
	}
}
extension SelectQuestionsFeature {
	public typealias HostingFeature = Self
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("SelectQuestionsFeature").font(.largeTitle)
			}
		}
	}
}

@Reducer
public struct AnswerSecurityQuestionFeature {
	public struct State: Equatable {}
	public enum Action {
		case delegate(DelegateAction)
		public enum DelegateAction {
			case done
		}
	}
}
extension AnswerSecurityQuestionFeature {
	public typealias HostingFeature = Self
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("AnswerSecurityQuestionFeature").font(.largeTitle)
			}
		}
	}
}

@Reducer
public struct SecurityQuestionsCreationCompleted {
	public struct State: Equatable {}
	public enum Action {
		case delegate(DelegateAction)
		public enum DelegateAction {
			case done
		}
	}
}
extension SecurityQuestionsCreationCompleted {
	public typealias HostingFeature = Self
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("SecurityQuestionsCreationCompleted").font(.largeTitle)
			}
		}
	}
}



@Reducer
public struct NewSecurityQuestionsFeatureCoordinator {
	
	@Reducer(state: .equatable)
	public enum Path {
		case answerQuestion0(AnswerSecurityQuestionFeature)
		case answerQuestion1(AnswerSecurityQuestionFeature)
		case answerQuestion2(AnswerSecurityQuestionFeature)
		case answerQuestion3(AnswerSecurityQuestionFeature)

		case creationCompleted(SecurityQuestionsCreationCompleted)
	}
	
	
	@ObservableState
	public struct State: Equatable {
		public var selectQuestions: SelectQuestionsFeature.State
		public var path = StackState<Path.State>()
		
		public init() {
			self.selectQuestions = SelectQuestionsFeature.State()
		}
	}
	
	@CasePathable
	public enum Action {
		@CasePathable
		public enum DelegateAction {
			case done
		}
		
		case path(StackAction<Path.State, Path.Action>)
		case selectQuestions(SelectQuestionsFeature.Action)
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.selectQuestions, action: \.selectQuestions) {
			SelectQuestionsFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .path(let pathAction):
//				switch pathAction {
//
//				case .element(id: _, action: .answerQuestion0(.delegate(.done))):
//					return .send(.delegate(.done))
//					
//				case .popFrom(id: _):
//					return .none
//				case .push(id: _, state: _):
//					return .none
//				default:
//					return .none
//				}
				return .none
				
			case .selectQuestions(.delegate(.done)):
				return .none
				
			
			case .selectQuestions(_):
				return .none
		
			case .delegate:
				return .none
				
			default:
				return .none
			}
		}
		.forEach(\.path, action: \.path)
	}
}

extension NewSecurityQuestionsFeatureCoordinator {
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<NewSecurityQuestionsFeatureCoordinator>
		
		public init(store: StoreOf<NewSecurityQuestionsFeatureCoordinator>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				SelectQuestionsFeature.View(
					store: store.scope(state: \.selectQuestions, action: \.selectQuestions)
				)
			} destination: { store in
				switch store.case {
				case let .answerQuestion0(store):
					AnswerSecurityQuestionFeature.View(store: store)
				case let .answerQuestion1(store):
					AnswerSecurityQuestionFeature.View(store: store)
				case let .answerQuestion2(store):
					AnswerSecurityQuestionFeature.View(store: store)
				case let .answerQuestion3(store):
					AnswerSecurityQuestionFeature.View(store: store)
					
				case let .creationCompleted(store):
					SecurityQuestionsCreationCompleted.View(store: store)
				}
			}
		}
	}
}
