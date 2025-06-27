package com.radixdlt.sargon.os.interactor

import com.radixdlt.sargon.AuthorizationPurpose
import com.radixdlt.sargon.AuthorizationResponse
import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.FactorSource
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.SignRequestOfAuthIntent
import com.radixdlt.sargon.SignRequestOfSubintent
import com.radixdlt.sargon.SignRequestOfTransactionIntent
import com.radixdlt.sargon.SignResponseOfAuthIntentHash
import com.radixdlt.sargon.SignResponseOfSubintentHash
import com.radixdlt.sargon.SignResponseOfTransactionIntentHash
import com.radixdlt.sargon.SpotCheckResponse

class FakeHostInteractor: HostInteractor {
    override suspend fun signTransactions(
        request: SignRequestOfTransactionIntent
    ): SignResponseOfTransactionIntentHash {
        throw CommonException.HostInteractionAborted()
    }

    override suspend fun signSubintents(request: SignRequestOfSubintent): SignResponseOfSubintentHash {
        throw CommonException.HostInteractionAborted()
    }

    override suspend fun deriveKeys(request: KeyDerivationRequest): KeyDerivationResponse {
        throw CommonException.HostInteractionAborted()
    }

    override suspend fun signAuth(request: SignRequestOfAuthIntent): SignResponseOfAuthIntentHash {
        throw CommonException.HostInteractionAborted()
    }

    override suspend fun requestAuthorization(purpose: AuthorizationPurpose): AuthorizationResponse {
        return AuthorizationResponse.REJECTED
    }

    override suspend fun spotCheck(
        factorSource: FactorSource,
        allowSkip: Boolean
    ): SpotCheckResponse {
        throw CommonException.HostInteractionAborted()
    }

}