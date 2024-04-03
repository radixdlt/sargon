package com.radixdlt.sargon.samples

import com.radixdlt.sargon.annotation.UsesSampleValues
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

@UsesSampleValues
val acedBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleAced()

@UsesSampleValues
val babeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleBabe()

@UsesSampleValues
val cafeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleCafe()

@UsesSampleValues
val deadBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleDead()

@UsesSampleValues
val ecadBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleEcad()

@UsesSampleValues
val fadeBagOfBytesSample: BagOfBytes
    get() = newBagOfBytesSampleFade()

@UsesSampleValues
fun BagOfBytes.appendingCafeSample() = bagOfBytesAppendCafe(to = this)

@UsesSampleValues
fun BagOfBytes.appendingDeadbeefSample() = bagOfBytesAppendDeadbeef(to = this)

@UsesSampleValues
fun BagOfBytes.prependingCafeSample() = bagOfBytesPrependCafe(inFrontOf = this)

@UsesSampleValues
fun BagOfBytes.prependingDeadbeefSample() = bagOfBytesPrependDeadbeef(inFrontOf = this)
