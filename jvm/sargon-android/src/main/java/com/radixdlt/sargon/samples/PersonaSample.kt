package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.newPersonaSampleMainnetBatman
import com.radixdlt.sargon.newPersonaSampleMainnetSatoshi
import com.radixdlt.sargon.newPersonaSampleStokenetHermione
import com.radixdlt.sargon.newPersonaSampleStokenetLeiaSkywalker

@UsesSampleValues
object PersonaSampleMainnet: Sample<Persona> {
    override fun invoke(): Persona = batman

    override fun other(): Persona = satoshi

    val batman: Persona
        get() = newPersonaSampleMainnetBatman()

    val satoshi: Persona
        get() = newPersonaSampleMainnetSatoshi()
}

@UsesSampleValues
val Persona.Companion.sampleMainnet: PersonaSampleMainnet
    get() = PersonaSampleMainnet

@UsesSampleValues
object PersonaSampleStokenet: Sample<Persona> {
    override fun invoke(): Persona = leiaSkywalker

    override fun other(): Persona = hermione

    val leiaSkywalker: Persona
        get() = newPersonaSampleStokenetLeiaSkywalker()

    val hermione: Persona
        get() = newPersonaSampleStokenetHermione()
}

@UsesSampleValues
val Persona.Companion.sampleStokenet: PersonaSampleStokenet
    get() = PersonaSampleStokenet