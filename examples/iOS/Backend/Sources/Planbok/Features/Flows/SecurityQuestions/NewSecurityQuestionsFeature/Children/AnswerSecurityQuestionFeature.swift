import ComposableArchitecture
import Foundation
import Sargon

// MARK: - AnswerSecurityQuestionFeature
@Reducer
public struct AnswerSecurityQuestionFeature {
	@ObservableState
	public struct State: Equatable {
		@Shared(.pendingAnswers) var pendingAnswers
		@Shared(.questions) var questions

		public let index: Int
		public var answer: String = ""
		public var trimmed: String {
			trimSecurityQuestionsAnswer(answer: answer)
		}

		public var question: SecurityNotProductionReadyQuestion {
			questions[index]
		}

		public init(index: Int, answer: String) {
			self.index = index
			self.answer = answer
		}

		public init(index: Int) {
			self.init(index: index, answer: "")
			self.answer = pendingAnswers[id: questions[index].id]?.answer ?? ""
		}
	}

	@CasePathable
	public enum Action: ViewAction {
		case delegate(DelegateAction)
		case view(ViewAction)
		public enum DelegateAction {
			case done(Int)
		}

		@CasePathable
		public enum ViewAction {
			case answerChanged(String)
			case confirmButtonTapped
		}
	}

	public var body: some ReducerOf<Self> {
		Reduce { state, action in
			switch action {
			case let .view(.answerChanged(answer)):
				state.answer = answer
				return .none
			case .view(.confirmButtonTapped):
				guard !state.trimmed.isEmpty else { return .none }
				state.pendingAnswers[id: state.question.id] = PendingAnswerToQuestion(questionID: state.question.id, answer: state.trimmed)
				return .send(.delegate(.done(state.index)))
			case .delegate:
				return .none
			}
		}
	}
}

extension AnswerSecurityQuestionFeature {
	public typealias HostingFeature = Self

	@ViewAction(for: HostingFeature.self)
	public struct View: SwiftUI.View {
		@Bindable public var store: StoreOf<HostingFeature>
		public init(store: StoreOf<HostingFeature>) {
			self.store = store
		}

		public var body: some SwiftUI.View {
			VStack {
				Text("Question #\(store.state.index)")
					.font(.largeTitle)

				Spacer()

				Text("\(store.state.question.question)")
					.font(.title)
					.fontWeight(.bold)

				Spacer()

				if
					case let unsafeAnswers = store.state.question.expectedAnswerFormat.unsafeAnswers,
					!unsafeAnswers.isEmpty
				{
					Text("Unsuitable if your answer would be: \(unsafeAnswers.map { "\"\($0)\"" }.joined(separator: ", "))")
						.foregroundStyle(Color.red)
				}

				LabeledTextField(
					label: "Answer",
					text: $store.answer.sending(\.view.answerChanged),
					placeholder: "Suggested format: `\(store.state.question.expectedAnswerFormat.answerStructure)`",
					hint: "Example: *\"\(store.state.question.expectedAnswerFormat.exampleAnswer)\"*"
				)
				.padding(.vertical, 20)

				Labeled("Used", "'\(store.state.trimmed)'")

				Spacer()

				Button("Confirm") {
					send(.confirmButtonTapped)
				}
				.buttonStyle(.borderedProminent)
				.disabled(store.state.answer.isEmpty)
			}
			.multilineTextAlignment(.leading)
			.padding()
		}
	}
}
