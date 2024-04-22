package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.HierarchicalDeterministicPublicKey
import com.radixdlt.sargon.Signature
import com.radixdlt.sargon.hierarchicalDeterministicPublicKeyIsValidSignatureForHash

fun HierarchicalDeterministicPublicKey.isValidSignature(
    signature: Signature,
    hash: Hash
) = hierarchicalDeterministicPublicKeyIsValidSignatureForHash(
    key = this,
    signature = signature,
    hash = hash
)