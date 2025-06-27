package com.radixdlt.sargon.extensions

import com.radixdlt.sargon.HierarchicalDeterministicFactorInstance
import com.radixdlt.sargon.Persona
import com.radixdlt.sargon.personaUnsecuredControllingFactorInstance

val Persona.unsecuredControllingFactorInstance: HierarchicalDeterministicFactorInstance?
    get() = personaUnsecuredControllingFactorInstance(persona = this)