package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.FactorInstanceBadge
import com.radixdlt.sargon.FactorInstanceBadgeVirtualSource
import com.radixdlt.sargon.MfaFactorInstance
import com.radixdlt.sargon.NonFungibleGlobalId
import com.radixdlt.sargon.nonFungibleGlobalFromHierarchicalDeterministicPublicKey

@Throws(SargonException::class)
fun MfaFactorInstance.nonFungibleGlobalId(): NonFungibleGlobalId {
    return when (val badge = factorInstance.badge) {
        is FactorInstanceBadge.Virtual -> when (val key = badge.value) {
            is FactorInstanceBadgeVirtualSource.HierarchicalDeterministic -> {
                nonFungibleGlobalIdFromHierarchicalDeterministicPublicKey(key.value)
            }
        }
    }
}