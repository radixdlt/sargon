//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-03.
//

import Foundation
import SwiftUI
import Sargon
import ComposableArchitecture

extension SecurityNotProductionReadyQuestionAndAnswer: Identifiable {
	public typealias ID = SecurityNotProductionReadyQuestion.ID
	public var id: ID {
		question.id
	}
}

public typealias AnswersToQuestions = IdentifiedArrayOf<SecurityNotProductionReadyQuestionAndAnswer>

@Reducer
public struct SecurityQuestionsReviewAnswersFeature {

	@Dependency(FactorSourcesClient.self) var factorSourcesClient
	
	@ObservableState
	public struct State: Equatable {
		public let answersToQuestions: AnswersToQuestions
		public var isAdding = false
	}

	@CasePathable
	public enum Action: ViewAction {
		case delegate(DelegateAction)
		case view(ViewAction)
		public enum DelegateAction {
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
					let factor = factorSourcesClient.createSecurityQuestionsFactor(qas)
                    try await factorSourcesClient.addFactorSource(factor.asGeneral)
                    await send(.delegate(.factorCreatedAndAdded))
				}
            case .delegate(_):
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
        .frame(maxWidth:. infinity)
        .padding()
        .background(Color.green)
        .clipShape(.rect(cornerRadius: 20))
    }
}
