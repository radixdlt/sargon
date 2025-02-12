import ComposableArchitecture
import Foundation
import Sargon
import SwiftUI

// MARK: - SecurityNotProductionReadyQuestionAndAnswer + Identifiable
extension SecurityNotProductionReadyQuestionAndAnswer: Identifiable {
	public typealias ID = SecurityNotProductionReadyQuestion.ID
	public var id: ID {
		question.id
	}
}

public typealias AnswersToQuestions = IdentifiedArrayOf<SecurityNotProductionReadyQuestionAndAnswer>

// MARK: - SecurityQuestionsReviewAnswersFeature
@Reducer
public struct SecurityQuestionsReviewAnswersFeature {
	@Dependency(FactorSourcesClient.self) var factorSourcesClient

	@ObservableState
	public struct State: Equatable {
		@Shared(.pendingAnswers) var toWipeAnswers
		public let answersToQuestions: AnswersToQuestions
		public var isAdding = false
	}

	@CasePathable
	public enum Action: ViewAction {
		case delegate(DelegateAction)
		case view(ViewAction)
		case `internal`(InternalAction)
		public enum DelegateAction {
			case factorCreatedAndAdded
		}

		public enum InternalAction {
			case factorCreatedAndAdded
		}

		@CasePathable
		public enum ViewAction {
			case addFactorButtonTapped
		}
	}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.addFactorButtonTapped):
				guard !state.isAdding else { return .none }
				state.isAdding = true
				return .run { [qas = state.answersToQuestions] send in
					let factor = try factorSourcesClient.createSecurityQuestionsFactor(qas)
					try await factorSourcesClient.addFactorSource(factor.asGeneral)
					await send(.internal(.factorCreatedAndAdded))
				}

			case .internal(.factorCreatedAndAdded):
				state.toWipeAnswers = [] // IMPORTANT! Since this is shared state (in memory) we SHOULD wipe secrets
				return .send(.delegate(.factorCreatedAndAdded))

			case .delegate:
				return .none
			}
		}
	}
}

extension SecurityQuestionsReviewAnswersFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("Review Answers").font(.largeTitle)
				ScrollView {
					ForEach(store.state.answersToQuestions) { answerToQuestion in
						let index = store.state.answersToQuestions.firstIndex(of: answerToQuestion)!
						AnsweredQuestionCard(answerToQuestion, index)
					}
					.multilineTextAlignment(.leading)
				}

				Button("Add Factor") {
					send(.addFactorButtonTapped)
				}
				.disabled(store.state.isAdding)
				.buttonStyle(.borderedProminent)
			}
			.padding()
		}
	}
}

// MARK: - AnsweredQuestionCard
public struct AnsweredQuestionCard: SwiftUI.View {
	public let answerToQuestion: AnswersToQuestions.Element
	public let index: Int
	public init(
		_ answerToQuestion: AnswersToQuestions.Element,
		_ index: Int
	) {
		self.answerToQuestion = answerToQuestion
		self.index = index
	}

	public var body: some View {
		VStack(alignment: .leading, spacing: 20) {
			Labeled("Question \(index)", answerToQuestion.question.question, axis: .vertical)
			Labeled("Answer \(index)", answerToQuestion.answer, axis: .vertical)
		}
		.fontWeight(.bold)
		.foregroundStyle(Color.white)
		.frame(maxWidth: .infinity)
		.padding()
		.background(Color.green)
		.clipShape(.rect(cornerRadius: 20))
	}
}
