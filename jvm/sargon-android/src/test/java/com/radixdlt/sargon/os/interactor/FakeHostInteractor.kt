package com.radixdlt.sargon.os.interactor

import com.radixdlt.sargon.AuthenticationSigningRequest
import com.radixdlt.sargon.AuthenticationSigningResponse
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.NeglectFactorReason
import com.radixdlt.sargon.NeglectedFactors
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent
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

    override suspend fun signAuth(request: AuthenticationSigningRequest): AuthenticationSigningResponse {
        throw CommonException.SigningRejected()
    }
}