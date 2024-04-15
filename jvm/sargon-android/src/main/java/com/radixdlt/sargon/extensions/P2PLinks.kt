package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.Hash
import com.radixdlt.sargon.P2pLink
import com.radixdlt.sargon.P2pLinks
import com.radixdlt.sargon.newP2PLinks
import com.radixdlt.sargon.newP2PLinksByAppending
import com.radixdlt.sargon.newP2PLinksRemovedById
import com.radixdlt.sargon.newP2PLinksRemovedElement
import com.radixdlt.sargon.p2PLinksElementCount
import com.radixdlt.sargon.p2PLinksGetElements
import com.radixdlt.sargon.p2PLinksGetP2PLinkById

fun P2pLinks.Companion.init(vararg p2pLink: P2pLink): P2pLinks =
    init(p2pLinks = p2pLink.asList())

fun P2pLinks.Companion.init(p2pLinks: List<P2pLink>): P2pLinks = newP2PLinks(p2PLinks = p2pLinks)

operator fun P2pLinks.invoke() = p2PLinksGetElements(p2PLinks = this)

operator fun P2pLinks.get(index: Int) = invoke().get(index = index)

operator fun P2pLinks.contains(element: P2pLink) = invoke().contains(element = element)

val P2pLinks.size: Int
    get() = p2PLinksElementCount(p2PLinks = this).toInt()

fun P2pLinks.append(p2pLink: P2pLink): P2pLinks =
    newP2PLinksByAppending(p2PLink = p2pLink, to = this)

fun P2pLinks.removeById(hash: Hash): P2pLinks =
    newP2PLinksRemovedById(idOfP2PLink = hash, from = this)

fun P2pLinks.remove(p2pLink: P2pLink): P2pLinks =
    newP2PLinksRemovedElement(p2PLink = p2pLink, from = this)

fun P2pLinks.getBy(hash: Hash): P2pLink? =
    p2PLinksGetP2PLinkById(p2PLinks = this, id = hash)