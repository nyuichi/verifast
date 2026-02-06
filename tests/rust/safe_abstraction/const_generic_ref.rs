#![allow(dead_code)]

struct Marker<const N: usize>;

fn id_ref<const N: usize>(a: &Marker<N>) -> &Marker<N> {
    a
}
