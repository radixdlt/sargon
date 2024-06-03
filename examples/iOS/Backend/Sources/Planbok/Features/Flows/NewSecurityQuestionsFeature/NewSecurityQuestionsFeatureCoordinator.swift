import SwiftUI
import Sargon
import ComposableArchitecture

@Reducer
public struct NewSecurityQuestionsFeatureCoordinator {
	
	@Reducer(state: .equatable)
	public enum Path {
		case answerQuestion(AnswerSecurityQuestionFeature)
		case reviewAnswers(SecurityQuestionsReviewAnswersFeature)
	}
	
	
	@ObservableState
	public struct State: Equatable {
		@Shared(.selectedQuestions) var selectedQuestions
		@Shared(.pendingAnswers) var pendingAnswers
		
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
	
	func nextStep(_ state: inout State, nextIndex indexOfNextQuestionToAnswer: Int) -> EffectOf<Self> {
		if indexOfNextQuestionToAnswer < state.selectedQuestions.count {
			state.path.append(.answerQuestion(
				AnswerSecurityQuestionFeature.State(
					index: indexOfNextQuestionToAnswer
				)
			))
		} else {
			precondition(state.pendingAnswers.count == state.selectedQuestions.count)
			let answersToQuestionsArray = state.pendingAnswers.map({
				let question = state.selectedQuestions[id: $0.key]!
				return SecurityNotProductionReadyQuestionAndAnswer(question: question, answer: $0.value)
			})
			let answersToQuestions = answersToQuestionsArray.asIdentified()
			state.path.append(.reviewAnswers(SecurityQuestionsReviewAnswersFeature.State(
				answersToQuestions: answersToQuestions
			)))
		}
			
	
		return .none
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.selectQuestions, action: \.selectQuestions) {
			SelectQuestionsFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .path(let pathAction):
				switch pathAction {

				case let .element(id: _, action: .answerQuestion(.delegate(.done(index)))):
					return nextStep(&state, nextIndex: index + 1)
					
				case .element(id: _, action: .reviewAnswers(.delegate(.factorCreatedAndAdded))):
					return .send(.delegate(.done))
					
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				default:
					return .none
				}
				
			case let .selectQuestions(.delegate(.done(prefillWith))):
				if let qas = prefillWith {
					state.selectedQuestions = qas.map(\.question).asIdentified()
					state.pendingAnswers = Dictionary(
						uniqueKeysWithValues: qas.map({ ($0.question.id, $0.answer) })
					)

					state.path = StackState(
						(0..<qas.count)
							.map(AnswerSecurityQuestionFeature.State.init(index:))
							.map(Path.State.answerQuestion)
					)
					
					state.path.append(.reviewAnswers(
						SecurityQuestionsReviewAnswersFeature.State(
							answersToQuestions: qas.asIdentified())
					))
					
					return .none
				} else {
					return nextStep(&state, nextIndex: 0)
				}
			
			case .selectQuestions(_):
				return .none
		
			case .delegate:
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
				case let .answerQuestion(store):
					AnswerSecurityQuestionFeature.View(store: store)
			
				case let .reviewAnswers(store):
					SecurityQuestionsReviewAnswersFeature.View(store: store)
				}
			}
		}
	}
}
