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
				Text("SecurityQuestionsReviewAnswersFeature").font(.largeTitle)
				ScrollView {
					ForEach(store.state.answersToQuestions) { answerToQuestion in
						VStack {
							Text("\(answerToQuestion.question.question)")
							Text("\(answerToQuestion.answer)").fontWeight(.bold)
						}
					}
				}
				
				Button("Add Factor") {
					send(.addFactorButtonTapped)
				}
			}
			.multilineTextAlignment(.leading)
		}
	}
}
