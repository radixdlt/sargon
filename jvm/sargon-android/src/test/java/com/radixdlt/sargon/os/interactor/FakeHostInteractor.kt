package com.radixdlt.sargon.os.interactor

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.SignRequestOfAuthIntent
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash

class FakeHostInteractor: HostInteractor {
    override suspend fun signTransactions(
        request: SignRequestOfTransactionIntent
    ): SignResponseOfTransactionIntentHash {
        throw CommonException.SigningRejected()
    }

    override suspend fun signSubintents(request: SignRequestOfSubintent): SignResponseOfSubintentHash {
        throw CommonException.SigningRejected()
    }

    override suspend fun deriveKeys(request: KeyDerivationRequest): KeyDerivationResponse {
        throw CommonException.SigningRejected()
    }

    override suspend fun signAuth(request: SignRequestOfAuthIntent): SignResponseOfAuthIntentHash {
        throw CommonException.SigningRejected()
    }

}