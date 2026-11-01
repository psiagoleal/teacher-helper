use std::time::Instant;

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "prova.pdf".into());
    let svg = std::env::args().nth(2).map(|p| std::fs::read(p).expect("svg"));
    let diagram = svg.as_deref().map(|b| (b, "Fluxograma: início, decisão x > 0, fim"));

    let t = Instant::now();
    match spike_typst::render_pdf(spike_typst::SAMPLES, diagram) {
        Ok(pdf) => {
            std::fs::write(&out, &pdf).unwrap();
            println!("PDF ok: {} bytes em {:?} -> {out}", pdf.len(), t.elapsed());
        }
        Err(e) => {
            eprintln!("ERRO: {e}");
            std::process::exit(1);
        }
    }
}
