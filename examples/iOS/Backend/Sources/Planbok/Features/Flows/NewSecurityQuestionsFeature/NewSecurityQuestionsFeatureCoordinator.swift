import SwiftUI
import Sargon
import ComposableArchitecture

extension SecurityNotProductionReadyQuestion: Identifiable {
	public typealias ID = UInt16
}
extension SecurityNotProductionReadyQuestion {
	public static let all: [SecurityNotProductionReadyQuestion] = securityQuestionsAll()
}



extension PersistenceReaderKey
where Self == PersistenceKeyDefault<InMemoryKey<AnswersToQuestions>> {
	static var answers: Self {
		PersistenceKeyDefault(
			.inMemory("answers"),
			[:]
		)
	}
}

public typealias AnswersToQuestions = [SecurityNotProductionReadyQuestion.ID: String?]

extension PersistenceReaderKey
where Self == PersistenceKeyDefault<InMemoryKey<IdentifiedArrayOf<SecurityNotProductionReadyQuestion>>> {
	static var selectedQuestions: Self {
		PersistenceKeyDefault(
			.inMemory("selectedQuestions"),
			[]
		)
	}
}

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
				VStack {
					Text("\(question.question)").font(.headline).fontWeight(.bold)
					if case let unsafeAnswers = question.expectedAnswerFormat.unsafeAnswers, !unsafeAnswers.isEmpty {
						Text("Dont select if: \(unsafeAnswers.joined(separator: ","))")
							.font(.footnote)
							.foregroundStyle(Color.red)
					}
				}
			}
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

@Reducer
public struct AnswerSecurityQuestionFeature {

	@ObservableState
	public struct State: Equatable {
		
		@Shared(.answers) var answers
		@Shared(.selectedQuestions) var selectedQuestions
		
		public let index: Int
		public var answer: String = ""
		public var question: SecurityNotProductionReadyQuestion {
			selectedQuestions[index]
		}
		
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
				guard !state.answer.isEmpty else { return .none }
				state.answers[state.question.id] = state.answer
				return .send(.delegate(.done))
			case .delegate(_):
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
					.font(.title).font(.body)
				
				Labeled("Structure", store.state.question.expectedAnswerFormat.answerStructure)
				
				LabeledTextField(label: "Answer", text: $store.answer.sending(\.view.answerChanged))
					.padding(.vertical, 20)
				
				Labeled("Example", store.state.question.expectedAnswerFormat.exampleAnswer)
				
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

@Reducer
public struct SecurityQuestionsCreationCompleted {

	@ObservableState
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
		@Shared(.selectedQuestions) var selectedQuestions
		@Shared(.answers) var answers
		
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
				switch pathAction {

				case .element(id: _, action: .answerQuestion0(.delegate(.done))):
					return .send(.delegate(.done))
					
				case .popFrom(id: _):
					return .none
				case .push(id: _, state: _):
					return .none
				default:
					return .none
				}
				
			case .selectQuestions(.delegate(.done)):
				state.answers = Dictionary(uniqueKeysWithValues: state.selectedQuestions.map({ ($0.id, String?.none) }))
				state.path.append(.answerQuestion0(
					AnswerSecurityQuestionFeature.State(
						index: 0
					)
				))
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
