package com.radixdlt.sargon.os.interactor

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.SignRequestOfAuthIntent
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent
import com.radixdlt.sargon.SignWithFactorsOutcomeOfAuthIntentHash
import com.radixdlt.sargon.SignWithFactorsOutcomeOfSubintentHash
import com.radixdlt.sargon.SignWithFactorsOutcomeOfTransactionIntentHash

class FakeHostInteractor: HostInteractor {
    override suspend fun signTransactions(
        request: SignRequestOfTransactionIntent
    ): SignWithFactorsOutcomeOfTransactionIntentHash {
        throw CommonException.SigningRejected()
    }

    override suspend fun signSubintents(request: SignRequestOfSubintent): SignWithFactorsOutcomeOfSubintentHash {
        throw CommonException.SigningRejected()
    }

    override suspend fun deriveKeys(request: KeyDerivationRequest): KeyDerivationResponse {
        throw CommonException.SigningRejected()
    }

    override suspend fun signAuth(request: SignRequestOfAuthIntent): SignWithFactorsOutcomeOfAuthIntentHash {
        throw CommonException.SigningRejected()
    }

}