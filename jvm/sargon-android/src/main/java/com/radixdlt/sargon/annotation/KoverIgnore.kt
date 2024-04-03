package com.radixdlt.sargon.annotation

/**
 * Annotation used to ignore certain elements from being included
 * into coverage report
 */
@MustBeDocumented
@Retention(AnnotationRetention.RUNTIME)
@Target(
    AnnotationTarget.TYPE,
    AnnotationTarget.FUNCTION,
    AnnotationTarget.CONSTRUCTOR,
    AnnotationTarget.CLASS,
    AnnotationTarget.PROPERTY,
    AnnotationTarget.PROPERTY_GETTER,
)
annotation class KoverIgnore
