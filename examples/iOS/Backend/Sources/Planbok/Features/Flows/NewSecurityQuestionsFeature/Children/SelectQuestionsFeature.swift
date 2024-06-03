//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-03.
//

import Foundation
import Sargon
import ComposableArchitecture


@Reducer
public struct SelectQuestionsFeature {

	@ObservableState
	public struct State: Equatable {
		@Shared(.selectedQuestions) var selectedQuestions
	}
	
	@CasePathable
	public enum Action: ViewAction {
		case delegate(DelegateAction)
		case view(ViewAction)
		public enum DelegateAction {
			case done
		}
		
		@CasePathable
		public enum ViewAction {
			case confirmedQuestions
		}
	}
	
	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case .view(.confirmedQuestions):
				return .send(.delegate(.done))
			case .delegate(_):
				return .none
			}
		}
	}
}

public struct SelectQuestionCard: View {
	@Shared(.selectedQuestions) var selectedQuestions
	public let question: SecurityNotProductionReadyQuestion
	public var id: SecurityNotProductionReadyQuestion.ID {
		question.id
	}
	public var isSelected: Bool {
		selectedQuestions[id: id] != nil
	}
	public var body: some SwiftUI.View {
		Button(action: {
			if isSelected {
				selectedQuestions.remove(id: id)
			} else {
				selectedQuestions.append(question)
			}
		}, label: {
			HStack {
				Text(isSelected ? "✅" : "☑️").font(.title)
                VStack(alignment: .leading) {
					Text("\(question.question)").font(.headline).fontWeight(.bold)
					if case let unsafeAnswers = question.expectedAnswerFormat.unsafeAnswers, !unsafeAnswers.isEmpty {
						Text("Unsuitable if: \(unsafeAnswers.joined(separator: ","))")
							.font(.footnote)
							.foregroundStyle(Color.red)
					}
				}
			}
            .multilineTextAlignment(.leading)
		})
		.buttonStyle(.plain)
		.frame(maxWidth: .infinity, alignment: .leading)
		.cornerRadius(.small1)
	}
}

extension SelectQuestionsFeature {
	public typealias HostingFeature = Self
	
	
	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		let amount = 4
		public let store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}
		public var body: some SwiftUI.View {
			VStack {
				Text("Pick #\(amount) questions").font(.title)
				Text("Picked: \(store.state.selectedQuestions.count)")
				ScrollView {
					ForEach(SecurityNotProductionReadyQuestion.all) { question in
						SelectQuestionCard(question: question)
							.padding(.bottom, 10)
					}
				}
				.padding(.vertical, 10)
				
				Button("Confirm Questions") {
					send(.confirmedQuestions)
				}
				.buttonStyle(.borderedProminent)
				.disabled(store.state.selectedQuestions.count != amount)
			}
			.padding()
		}
	}
}

