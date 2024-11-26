package com.radixdlt.sargon.os.interactor

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.HostInteractor
import com.radixdlt.sargon.KeyDerivationRequest
import com.radixdlt.sargon.KeyDerivationResponse
import com.radixdlt.sargon.NeglectFactorReason
import com.radixdlt.sargon.NeglectedFactors
import com.radixdlt.sargon.SignRequestForSubintent
import com.radixdlt.sargon.SignRequestForTransactionIntent
import com.radixdlt.sargon.SignWithFactorsOutcomeForSubintent
import com.radixdlt.sargon.SignWithFactorsOutcomeForTransactionIntent

class FakeHostInteractor: HostInteractor {
    override suspend fun signTransactions(
        request: SignRequestForTransactionIntent
    ): SignWithFactorsOutcomeForTransactionIntent {
        return SignWithFactorsOutcomeForTransactionIntent.Neglected(
            v1 = NeglectedFactors(
                reason = NeglectFactorReason.FAILURE,
                factors = request.perFactorSource.keys.toList()
            )
        )
    }

    override suspend fun signSubintents(request: SignRequestForSubintent): SignWithFactorsOutcomeForSubintent {
        return SignWithFactorsOutcomeForSubintent.Neglected(
            v1 = NeglectedFactors(
                reason = NeglectFactorReason.FAILURE,
                factors = request.perFactorSource.keys.toList()
            )
        )
    }

    override suspend fun deriveKeys(request: KeyDerivationRequest): KeyDerivationResponse {
        throw CommonException.Unknown()
    }
}