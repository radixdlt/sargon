package com.radixdlt.sargon.samples

import com.radixdlt.sargon.CommonException
import com.radixdlt.sargon.annotation.UsesSampleValues
import com.radixdlt.sargon.extensions.SargonException

@UsesSampleValues
val sargonExceptionSample: Sample<SargonException>
    get() = object : Sample<SargonException> {
        override fun invoke(): SargonException = CommonException.Unknown("Sample error")

        override fun other(): SargonException = CommonException.BytesEmpty()
    }