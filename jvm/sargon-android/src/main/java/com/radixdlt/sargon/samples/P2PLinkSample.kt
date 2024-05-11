package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Ed25519PublicKey
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.RadixConnectPassword
import com.radixdlt.sargon.RadixConnectPurpose
import com.radixdlt.sargon.annotation.UsesSampleValues

@UsesSampleValues
val P2pLink.Companion.sample: Sample<P2pLink>
    get() = object : Sample<P2pLink> {

        override fun invoke(): P2pLink = P2pLink(
            connectionPassword = RadixConnectPassword.sample(),
            connectionPurpose = RadixConnectPurpose.sample(),
            publicKey = Ed25519PublicKey.sample(),
            displayName = "Sample"
        )

        override fun other(): P2pLink = P2pLink(
            connectionPassword = RadixConnectPassword.sample.other(),
            connectionPurpose = RadixConnectPurpose.sample.other(),
            publicKey = Ed25519PublicKey.sample.other(),
            displayName = "Sample Other"
        )
    }