const fn VeriFast_ghost_command() {}

fn VeriFast_alloc<T>() -> *mut T {
    VeriFast_alloc()
}

fn VF_free<T>(_ptr: *mut T) {}
