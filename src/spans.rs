use tracing::{info_span, Span};

#[inline(always)]
pub fn span_00_fields() -> Span {
    info_span!("foo")
}

#[inline(always)]
pub fn span_04_fields() -> Span {
    info_span!("foo", a = 1, b = 2, c = 3, d = 4)
}

#[inline(always)]
pub fn span_08_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8
    )
}

#[inline(always)]
pub fn span_12_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12
    )
}

#[inline(always)]
pub fn span_16_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12,
        m = 13,
        n = 14,
        o = 15,
        p = 16
    )
}

#[inline(always)]
pub fn span_20_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12,
        m = 13,
        n = 14,
        o = 15,
        p = 16,
        q = 17,
        r = 18,
        s = 19,
        t = 20
    )
}

#[inline(always)]
pub fn span_24_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12,
        m = 13,
        n = 14,
        o = 15,
        p = 16,
        q = 17,
        r = 18,
        s = 19,
        t = 20,
        u = 21,
        v = 22,
        w = 23,
        x = 24
    )
}

#[inline(always)]
pub fn span_28_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12,
        m = 13,
        n = 14,
        o = 15,
        p = 16,
        q = 17,
        r = 18,
        s = 19,
        t = 20,
        u = 21,
        v = 22,
        w = 23,
        x = 24,
        y = 25,
        z = 26,
        aa = 27,
        bb = 28
    )
}

#[inline(always)]
pub fn span_32_fields() -> Span {
    info_span!(
        "foo",
        a = 1,
        b = 2,
        c = 3,
        d = 4,
        e = 5,
        f = 6,
        g = 7,
        h = 8,
        i = 9,
        j = 10,
        k = 11,
        l = 12,
        m = 13,
        n = 14,
        o = 15,
        p = 16,
        q = 17,
        r = 18,
        s = 19,
        t = 20,
        u = 21,
        v = 22,
        w = 23,
        x = 24,
        y = 25,
        z = 26,
        aa = 27,
        bb = 28,
        cc = 29,
        dd = 30,
        ee = 31,
        ff = 32
    )
}
