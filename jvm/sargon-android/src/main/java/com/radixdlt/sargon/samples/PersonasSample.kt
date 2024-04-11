package com.radixdlt.sargon.samples

import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.Personas
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.init

@UsesSampleValues
val Personas.Companion.sampleMainnet: Sample<Personas>
    get() = object : Sample<Personas> {
        override fun invoke(): Personas = Personas.init(
            listOf(
                Persona.sampleMainnet.batman,
                Persona.sampleMainnet.satoshi
            )
        )

        override fun other(): Personas = Personas.init(Persona.sampleMainnet.satoshi)

    }

@UsesSampleValues
val Personas.Companion.sampleStokenet: Sample<Personas>
    get() = object : Sample<Personas> {
        override fun invoke(): Personas = Personas.init(
            listOf(
                Persona.sampleStokenet.leiaSkywalker,
                Persona.sampleStokenet.hermione
            )
        )

        override fun other(): Personas = Personas.init(Persona.sampleStokenet.hermione)
    }