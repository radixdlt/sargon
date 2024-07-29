package com.radixdlt.sargon

import com.radixdlt.sargon.extensions.default
import com.radixdlt.sargon.extensions.toDecimal192
import com.radixdlt.sargon.samples.Sample
import com.radixdlt.sargon.samples.sample
import org.junit.jupiter.api.Assertions.assertEquals
import org.junit.jupiter.api.Test

class AppPreferencesTest: SampleTestable<AppPreferences> {

    override val samples: List<Sample<AppPreferences>>
        get() = listOf(AppPreferences.sample)

    @Test
    fun testDefault() {
        assertEquals(
            AppPreferences(
                display = AppDisplay(
                    isCurrencyAmountVisible = true,
                    fiatCurrencyPriceTarget = FiatCurrency.USD
                ),
                gateways = SavedGateways.default,
                security = Security(
                    isCloudProfileSyncEnabled = true,
                    isDeveloperModeEnabled = false,
                    securityStructuresOfFactorSourceIds = emptyList()
                ),
                transaction = TransactionPreferences(defaultDepositGuarantee = 0.99.toDecimal192())
            ),
            AppPreferences.default()
        )
    }
}