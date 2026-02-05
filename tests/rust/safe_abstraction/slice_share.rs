#![allow(dead_code)]

unsafe fn test<'a, T>(s: &'a [T]) -> &'a [T]
//@ req type_interp::<T>() &*& [?q]lifetime_token(?a) &*& [_](<[T]>.share(a, ?t, s));
//@ ens type_interp::<T>() &*& [q]lifetime_token(a) &*& [_](<[T]>.share(a, t, result));
//@ on_unwind_ens false;
{
    s
}
