use bip0039::{Count, Mnemonic};

fn main() {
    let mnemonic = Mnemonic::generate(Count::Words12);
    let phrase = mnemonic.phrase();
    // let seed = mnemonic.to_seed();

    println!("{}", phrase);
}
