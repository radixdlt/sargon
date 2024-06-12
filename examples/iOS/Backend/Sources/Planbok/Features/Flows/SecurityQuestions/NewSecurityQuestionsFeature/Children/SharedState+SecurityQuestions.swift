//
//  File.swift
//  
//
//  Created by Alexander Cyon on 2024-06-03.
//

import Foundation
import Sargon
import ComposableArchitecture

extension SecurityNotProductionReadyQuestion: Identifiable {
	public typealias ID = UInt16
}
extension SecurityNotProductionReadyQuestion {
	public static let all: [SecurityNotProductionReadyQuestion] = securityQuestionsAll()
}


extension PersistenceReaderKey
where Self == PersistenceKeyDefault<InMemoryKey<PendingAnswersToQuestions>> {
	static var pendingAnswers: Self {
		PersistenceKeyDefault(
			.inMemory("pendingAnswers"),
			[]
		)
	}
}

public struct PendingAnswerToQuestion: Hashable, Sendable, Identifiable {
	public typealias ID = SecurityNotProductionReadyQuestion.ID
	public let questionID: ID
	public let answer: String
	
	public var id: ID { questionID }
}

public typealias PendingAnswersToQuestions = IdentifiedArrayOf<PendingAnswerToQuestion>

extension PersistenceReaderKey
where Self == PersistenceKeyDefault<InMemoryKey<IdentifiedArrayOf<SecurityNotProductionReadyQuestion>>> {
	static var questions: Self {
		PersistenceKeyDefault(
			.inMemory("questions"),
			[]
		)
	}
}
