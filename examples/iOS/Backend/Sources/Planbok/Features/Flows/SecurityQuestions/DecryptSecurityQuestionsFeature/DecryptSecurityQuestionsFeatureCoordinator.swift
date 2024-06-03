import SwiftUI
import Sargon
import ComposableArchitecture

@Reducer
public struct DecryptSecurityQuestionsFeatureCoordinator {
	
	@Dependency(FactorSourcesClient.self) var factorSourcesClient
	
	@Reducer(state: .equatable)
	public enum Path {
		case answerQuestion(AnswerSecurityQuestionFeature)
	}
	
	@ObservableState
	public struct State: Equatable {
		@Shared(.questions) var questions
		@Shared(.pendingAnswers) var pendingAnswers
		
		public let securityQuestionsFactorSource: SecurityQuestionsNotProductionReadyFactorSource
		public var firstQuestion: AnswerSecurityQuestionFeature.State
		public var path = StackState<Path.State>()
		
		public init(securityQuestionsFactorSource: SecurityQuestionsNotProductionReadyFactorSource) {
			self.securityQuestionsFactorSource = securityQuestionsFactorSource
			self.firstQuestion = AnswerSecurityQuestionFeature.State(index: 0, answer: "")
			self.questions = securityQuestionsFactorSource
				.sealedMnemonic
				.securityQuestions.asIdentified()
			self.pendingAnswers = [:]
		}
	}
	
	@CasePathable
	public enum Action {
		@CasePathable
		public enum DelegateAction {
			case done
		}
		
		case path(StackAction<Path.State, Path.Action>)
		case firstQuestion(AnswerSecurityQuestionFeature.Action)
		case delegate(DelegateAction)
	}
	
	public init() {}
	
	func nextStep(_ state: inout State, nextIndex indexOfNextQuestionToAnswer: Int) -> EffectOf<Self> {
		if indexOfNextQuestionToAnswer < state.questions.count {
			state.path.append(.answerQuestion(
				AnswerSecurityQuestionFeature.State(
					index: indexOfNextQuestionToAnswer
				)
			))
			return .none
		} else {
			precondition(state.pendingAnswers.count == state.questions.count)
			let answersToQuestionsArray = state.pendingAnswers.map({
				let question = state.questions[id: $0.key]!
				return SecurityNotProductionReadyQuestionAndAnswer(question: question, answer: $0.value)
			})
			let answersToQuestions = answersToQuestionsArray.asIdentified()
		
			do {
				let mnemonic = try factorSourcesClient.decryptSecurityQuestionsFactor(
					answersToQuestions,
					state.securityQuestionsFactorSource
				)
				log.info("Decrypted: \(mnemonic.phrase)")
				return .send(.delegate(.done))
			} catch {
				log.fault("Failed to decrypt SecurityQuestionsFactorSource with answers to questions, error: \(error)")
				return .send(.delegate(.done))
			}
		}
	}
	
	public var body: some ReducerOf<Self> {
		Scope(state: \.firstQuestion, action: \.firstQuestion) {
			AnswerSecurityQuestionFeature()
		}
		Reduce { state, action in
			switch action {
				
			case .path(let pathAction):
				switch pathAction {

				case let .element(id: _, action: .answerQuestion(.delegate(.done(index)))):
					return nextStep(&state, nextIndex: index + 1)
					
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				default:
					return .none
				}
				
			case let .firstQuestion(.delegate(.done(index))):
				return nextStep(&state, nextIndex: index + 1)
			
			case .firstQuestion:
				return .none
		
			case .delegate:
				return .none
				
			}
		}
		.forEach(\.path, action: \.path)
	}
}

extension DecryptSecurityQuestionsFeatureCoordinator {
	public struct View: SwiftUI.View {
		@Bindable var store: StoreOf<DecryptSecurityQuestionsFeatureCoordinator>
		
		public init(store: StoreOf<DecryptSecurityQuestionsFeatureCoordinator>) {
			self.store = store
		}
		
		public var body: some SwiftUI.View {
			NavigationStack(path: $store.scope(state: \.path, action: \.path)) {
				AnswerSecurityQuestionFeature.View(
					store: store.scope(state: \.firstQuestion, action: \.firstQuestion)
				)
			} destination: { store in
				switch store.case {
				case let .answerQuestion(store):
					AnswerSecurityQuestionFeature.View(store: store)
				}
			}
		}
	}
}
