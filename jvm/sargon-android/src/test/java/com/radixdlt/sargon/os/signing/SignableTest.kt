package com.radixdlt.sargon.os.signing

import com.radixdlt.sargon.AuthIntent
import com.radixdlt.sargon.AuthIntentHash
import com.radixdlt.sargon.CompiledSubintent
import com.radixdlt.sargon.CompiledTransactionIntent
import com.radixdlt.sargon.Subintent
import com.radixdlt.sargon.SubintentHash
import com.radixdlt.sargon.TransactionIntent
import com.radixdlt.sargon.TransactionIntentHash
import com.radixdlt.sargon.extensions.compile
import com.radixdlt.sargon.extensions.decompile
import com.radixdlt.sargon.extensions.hash
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Nested
import org.junit.jupiter.api.Test

class SignableTest {

    @Nested
    inner class IDTest {
        @Test
        fun transactionIntent() {
            val intentHash = TransactionIntentHash.sample()
            val id = Signable.ID.Transaction(intentHash)

            assertEquals(
                intentHash.hash,
                id.hash()
            )
        }

        @Test
        fun subintent() {
            val intentHash = SubintentHash.sample()
            val id = Signable.ID.Subintent(intentHash)

            assertEquals(
                intentHash.hash,
                id.hash()
            )
        }

        @Test
        fun authIntent() {
            val intentHash = AuthIntentHash.sample()
            val id = Signable.ID.Auth(intentHash)

            assertEquals(
                intentHash.payload.hash(),
                id.hash()
            )
        }
    }


    @Nested
    inner class PayloadTest {
        @Test
        fun transactionIntent() {
            val compiledIntent = CompiledTransactionIntent.sample()
            val payload = Signable.Payload.Transaction(compiledIntent)

            assertEquals(
                Signable.Transaction(compiledIntent.decompile()),
                payload.getSignable()
            )
        }

        @Test
        fun subintent() {
            val compiledIntent = CompiledSubintent.sample()
            val payload = Signable.Payload.Subintent(compiledIntent)

            assertEquals(
                Signable.Subintent(compiledIntent.decompile()),
                payload.getSignable()
            )
        }

        @Test
        fun authIntent() {
            val intentHash = AuthIntent.sample()
            val payload = Signable.Payload.Auth(intentHash)

            assertEquals(
                Signable.Auth(intentHash),
                payload.getSignable()
            )
        }
    }

    @Nested
    inner class SignableTest {
        @Test
        fun transactionIntent() {
            val intent = TransactionIntent.sample()
            val signable = Signable.Transaction(intent)

            assertEquals(
                Signable.Payload.Transaction(intent.compile()),
                signable.getPayload(),
            )

            assertEquals(
                Signable.ID.Transaction(intent.hash()),
                signable.getId()
            )
        }

        @Test
        fun subintent() {
            val intent = Subintent.sample()
            val signable = Signable.Subintent(intent)

            assertEquals(
                Signable.Payload.Subintent(intent.compile()),
                signable.getPayload(),
            )

            assertEquals(
                Signable.ID.Subintent(intent.hash()),
                signable.getId()
            )
        }

        @Test
        fun authIntent() {
            val intent = AuthIntent.sample()
            val signable = Signable.Auth(intent)

            assertEquals(
                Signable.Payload.Auth(intent),
                signable.getPayload(),
            )

            assertEquals(
                Signable.ID.Auth(intent.hash()),
                signable.getId()
            )
        }
    }
}