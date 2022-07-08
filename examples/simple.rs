use target_lexicon::Triple;
use target_lexicon_macros::triple;

const TARGET: Triple = triple!("unknown-nvidia-unknown");

fn main() {
    dbg!(TARGET);
}
