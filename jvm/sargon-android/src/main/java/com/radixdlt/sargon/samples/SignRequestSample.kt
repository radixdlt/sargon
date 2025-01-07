package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newSignRequestOfAuthIntentSample
import com.radixdlt.sargon.newSignRequestOfAuthIntentSampleOther
import com.radixdlt.sargon.newSignRequestOfSubintentSample
import com.radixdlt.sargon.newSignRequestOfSubintentSampleOther
import com.radixdlt.sargon.newSignRequestOfTransactionIntentSample
import com.radixdlt.sargon.newSignRequestOfTransactionIntentSampleOther
import com.radixdlt.sargon.os.signing.SignRequest
import com.radixdlt.sargon.os.signing.Signable.ID
import com.radixdlt.sargon.os.signing.Signable.Payload
import com.radixdlt.sargon.os.signing.into

@UsesSampleValues
val SignRequest.Companion.sampleTransactionIntent: Sample<SignRequest<Payload.Transaction, ID.Transaction>>
    get() = object : Sample<SignRequest<Payload.Transaction, ID.Transaction>> {
        override fun invoke(): SignRequest<Payload.Transaction, ID.Transaction>
                = newSignRequestOfTransactionIntentSample().into()


        override fun other(): SignRequest<Payload.Transaction, ID.Transaction>
                = newSignRequestOfTransactionIntentSampleOther().into()
    }

@UsesSampleValues
val SignRequest.Companion.sampleSubintent: Sample<SignRequest<Payload.Subintent, ID.Subintent>>
    get() = object : Sample<SignRequest<Payload.Subintent, ID.Subintent>> {
        override fun invoke(): SignRequest<Payload.Subintent, ID.Subintent>
                = newSignRequestOfSubintentSample().into()


        override fun other(): SignRequest<Payload.Subintent, ID.Subintent>
                = newSignRequestOfSubintentSampleOther().into()
    }

@UsesSampleValues
val SignRequest.Companion.sampleAuthIntent: Sample<SignRequest<Payload.Auth, ID.Auth>>
    get() = object : Sample<SignRequest<Payload.Auth, ID.Auth>> {
        override fun invoke(): SignRequest<Payload.Auth, ID.Auth>
                = newSignRequestOfAuthIntentSample().into()


        override fun other(): SignRequest<Payload.Auth, ID.Auth>
                = newSignRequestOfAuthIntentSampleOther().into()
    }