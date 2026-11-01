use math_core::{LatexToMathML, MathCoreConfig, MathDisplay};

fn main() {
    let conv = LatexToMathML::new(MathCoreConfig::default(), Vec::new()).expect("config");
    for line in std::io::stdin().lines() {
        let latex = line.unwrap();
        match conv.convert_with_local_state(&latex, MathDisplay::Inline) {
            Ok(r) => println!("OK\t{}\t{}", latex, r.mathml),
            Err(e) => println!("ERR\t{}\t{:?}", latex, e),
        }
    }
}
