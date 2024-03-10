package com.radixdlt.sargon.sample

import androidx.annotation.VisibleForTesting
import androidx.compose.ui.tooling.preview.PreviewParameterProvider
import com.radixdlt.sargon.AppPreferences
import com.radixdlt.sargon.BagOfBytes
import com.radixdlt.sargon.bagOfBytesAppendCafe
import com.radixdlt.sargon.bagOfBytesAppendDeadbeef
import com.radixdlt.sargon.bagOfBytesPrependCafe
import com.radixdlt.sargon.bagOfBytesPrependDeadbeef
import com.radixdlt.sargon.newBagOfBytesSampleAced
import com.radixdlt.sargon.newBagOfBytesSampleBabe
import com.radixdlt.sargon.newBagOfBytesSampleCafe
import com.radixdlt.sargon.newBagOfBytesSampleDead
import com.radixdlt.sargon.newBagOfBytesSampleEcad
import com.radixdlt.sargon.newBagOfBytesSampleFade

@VisibleForTesting
val acedBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleAced()

@VisibleForTesting
val babeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleBabe()

@VisibleForTesting
val cafeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleCafe()

@VisibleForTesting
val deadBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleDead()

@VisibleForTesting
val ecadBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleEcad()

@VisibleForTesting
val fadeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleFade()

@VisibleForTesting
fun BagOfBytes.appendingCafeSample() = bagOfBytesAppendCafe(to = this)

@VisibleForTesting
fun BagOfBytes.appendingDeadbeefSample() = bagOfBytesAppendDeadbeef(to = this)

@VisibleForTesting
fun BagOfBytes.prependingCafeSample() = bagOfBytesPrependCafe(inFrontOf = this)

@VisibleForTesting
fun BagOfBytes.prependingDeadbeefSample() = bagOfBytesPrependDeadbeef(inFrontOf = this)
