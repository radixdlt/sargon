package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.RolaChallenge
import com.radixdlt.sargon.rolaChallengeGetHash

fun RolaChallenge.hash(): Hash = rolaChallengeGetHash(rolaChallenge = this)