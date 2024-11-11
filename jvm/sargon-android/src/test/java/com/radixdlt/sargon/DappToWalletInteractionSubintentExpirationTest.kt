package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.status
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class DappToWalletInteractionSubintentExpirationTest {

    @Test
    fun testAfterDelayStatusValid() {
        assertEquals(
            DappToWalletInteractionSubintentExpirationStatus.VALID,
            DappToWalletInteractionSubintentExpiration.AfterDelay(
                DappToWalletInteractionSubintentExpireAfterDelay(expireAfterSeconds = 0u)
            ).status
        )
    }

    @Test
    fun testAtTimeInFutureStatusValid() {
        assertEquals(
            DappToWalletInteractionSubintentExpirationStatus.VALID,
            DappToWalletInteractionSubintentExpiration.AtTime(
                DappToWalletInteractionSubintentExpireAtTime(
                    unixTimestampSeconds = Timestamp.now().plusHours(1L).toEpochSecond().toULong()
                )
            ).status
        )
    }

    @Test
    fun testAtTimeInPastStatusExpired() {
        assertEquals(
            DappToWalletInteractionSubintentExpirationStatus.EXPIRED,
            DappToWalletInteractionSubintentExpiration.AtTime(
                DappToWalletInteractionSubintentExpireAtTime(
                    unixTimestampSeconds = Timestamp.now().minusSeconds(1L).toEpochSecond().toULong()
                )
            ).status
        )
    }

    @Test
    fun testAtTimeIn20SecStatusExpirationTooClose() {
        assertEquals(
            DappToWalletInteractionSubintentExpirationStatus.EXPIRATION_TOO_CLOSE,
            DappToWalletInteractionSubintentExpiration.AtTime(
                DappToWalletInteractionSubintentExpireAtTime(
                    unixTimestampSeconds = Timestamp.now().plusSeconds(20L).toEpochSecond().toULong()
                )
            ).status
        )
    }

}