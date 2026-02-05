#![allow(dead_code)]

/*@
lem slice_own_empty()
    req true;
    ens slice_own::<u8>()(default_tid, slice_of_elems(nil));
{
    close foreach(nil, (own)(default_tid));
    close slice_own::<u8>()(default_tid, slice_of_elems(nil));
}
@*/
