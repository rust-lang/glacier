enum E { A, }

const C: [u32; 1] = [1];

fn main() {
    let a = C[E::A as usize];
}
