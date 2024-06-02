import Foundation
import SargonUniFFI


extension SecurityQuestionsNotProductionReadyFactorSource {
	public init(
		mnemonic: Mnemonic,
		questionsAndAnswers: [SecurityNotProductionReadyQuestionAndAnswer]
	) {
		self = newSecurityQuestionsFactorSourceByEncryptingMnemonic(mnemonic: mnemonic, with: questionsAndAnswers)
	}
	
	public func decrypt(questionsAndAnswers: [SecurityNotProductionReadyQuestionAndAnswer]) throws -> Mnemonic {
		try securityQuestionsFactorSourceDecrypt(factorSource: self, with: questionsAndAnswers)
	}
}
