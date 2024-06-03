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
			[:]
		)
	}
}

public typealias PendingAnswersToQuestions = [SecurityNotProductionReadyQuestion.ID: String]

extension PersistenceReaderKey
where Self == PersistenceKeyDefault<InMemoryKey<IdentifiedArrayOf<SecurityNotProductionReadyQuestion>>> {
	static var questions: Self {
		PersistenceKeyDefault(
			.inMemory("questions"),
			[]
		)
	}
}
