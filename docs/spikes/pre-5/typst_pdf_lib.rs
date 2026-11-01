// Spike PRE-5: LaTeX (mitex) -> Typst -> PDF/UA-1, inteiramente em memória.

use std::collections::HashMap;

use typst::diag::{FileError, FileResult};
use typst::foundations::{Bytes, Datetime, Duration};
use typst_layout::PagedDocument;
use typst::syntax::{FileId, RootedPath, Source, VirtualPath, VirtualRoot};
use typst::text::{Font, FontBook};
use typst::utils::LazyHash;
use typst::{Library, LibraryExt, World};
use typst_pdf::{PdfOptions, PdfStandard, PdfStandards};

const SPEC_MOD: &str = include_str!("../assets/specs/mod.typ");
const SPEC_PRELUDE: &str = include_str!("../assets/specs/prelude.typ");
const SPEC_STANDARD: &str = include_str!("../assets/specs/latex/standard.typ");
const XARROW: &str = include_str!("../assets/xarrow.typ");

pub const SAMPLES: &[&str] = &[
    r"\frac{a+b}{2}",
    r"x = \frac{-b \pm \sqrt{b^2-4ac}}{2a}",
    r"\int_0^1 x^2\,dx = \frac{1}{3}",
    r"\sum_{i=1}^{n} i = \frac{n(n+1)}{2}",
    r"\begin{pmatrix} 1 & 2 \\ 3 & 4 \end{pmatrix}",
    r"\lim_{x \to 0} \frac{\sin x}{x} = 1",
    r"\vec{F} = m\vec{a}",
    r"\Delta H < 0 \Rightarrow \text{exotérmica}",
    r"\mathrm{H_2O} \rightleftharpoons \mathrm{H^+} + \mathrm{OH^-}",
    r"f(x) = \begin{cases} x^2 & x \ge 0 \\ -x & x < 0 \end{cases}",
    r"A \xrightarrow{\Delta} B",
];

fn id(path: &str) -> FileId {
    FileId::new(RootedPath::new(VirtualRoot::Project, VirtualPath::new(path).unwrap()))
}

struct MemWorld {
    library: LazyHash<Library>,
    book: LazyHash<FontBook>,
    fonts: Vec<Font>,
    sources: HashMap<FileId, Source>,
    files: HashMap<FileId, Bytes>,
    main: FileId,
}

impl MemWorld {
    fn new(main_text: String, files: Vec<(&str, Vec<u8>)>) -> Self {
        let fonts: Vec<Font> = typst_assets::fonts()
            .flat_map(|data| Font::iter(Bytes::new(data)))
            .collect();
        let main = id("/main.typ");
        let mut sources = HashMap::new();
        for (path, text) in [
            ("/main.typ", main_text),
            ("/specs/mod.typ", SPEC_MOD.to_string()),
            ("/specs/prelude.typ", SPEC_PRELUDE.to_string()),
            ("/specs/latex/standard.typ", SPEC_STANDARD.to_string()),
            ("/xarrow.typ", XARROW.to_string()),
        ] {
            let fid = id(path);
            sources.insert(fid, Source::new(fid, text));
        }
        let files = files.into_iter().map(|(p, b)| (id(p), Bytes::new(b))).collect();
        Self {
            library: LazyHash::new(Library::default()),
            book: LazyHash::new(FontBook::from_fonts(&fonts)),
            fonts,
            sources,
            files,
            main,
        }
    }
}

impl World for MemWorld {
    fn library(&self) -> &LazyHash<Library> {
        &self.library
    }
    fn book(&self) -> &LazyHash<FontBook> {
        &self.book
    }
    fn main(&self) -> FileId {
        self.main
    }
    fn source(&self, id: FileId) -> FileResult<Source> {
        self.sources
            .get(&id)
            .cloned()
            .ok_or_else(|| FileError::NotFound(id.vpath().get_with_slash().into()))
    }
    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if let Some(b) = self.files.get(&id) {
            return Ok(b.clone());
        }
        self.source(id).map(|s| Bytes::from_string(s.text().to_string()))
    }
    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }
    fn today(&self, _: Option<Duration>) -> Option<Datetime> {
        Datetime::from_ymd(2026, 11, 1)
    }
}

fn typst_string(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// Monta a fonte Typst de uma prova de exemplo a partir de fórmulas LaTeX.
pub fn exam_source(latex: &[&str], diagram_svg: Option<&str>) -> Result<String, String> {
    let mut s = String::from(
        r#"#import "/specs/mod.typ": mitex-scope
#let m(s, alt) = math.equation(block: false, alt: alt, eval("$" + s + "$", scope: mitex-scope).body)
#set document(title: "Prova de Matemática — spike PRE-5")
#set text(lang: "pt", region: "BR", size: 11pt)
#set page(paper: "a4", margin: 2cm, numbering: "1 / 1")
#set enum(numbering: "1.")

= Prova de Matemática
Escola Exemplo · Nome: #box(width: 1fr, repeat[\_])

"#,
    );
    for (i, l) in latex.iter().enumerate() {
        let typ = mitex::convert_math(l, None).map_err(|e| format!("{l}: {e}"))?;
        s += &format!(
            "+ Calcule ou interprete a expressão #m({}, {}) e justifique.\n  #block(breakable: false)[(a) x = {} #h(1em) (b) x = 0 #h(1em) (c) nenhuma]\n",
            typst_string(&typ),
            typst_string(l),
            i + 1
        );
    }
    s += "\n+ Leia o código e responda:\n```python\ndef media(xs):\n    return sum(xs) / len(xs)\n```\n";
    if let Some(alt) = diagram_svg {
        s += &format!(
            "\n+ Analise o fluxograma:\n  #image(\"/diagram.svg\", width: 70%, alt: {})\n",
            typst_string(alt)
        );
    }
    Ok(s)
}

/// Compila a prova para PDF/UA-1. Retorna os bytes do PDF ou os erros do Typst.
pub fn render_pdf(latex: &[&str], diagram: Option<(&[u8], &str)>) -> Result<Vec<u8>, String> {
    let source = exam_source(latex, diagram.map(|(_, alt)| alt))?;
    let files = diagram
        .map(|(svg, _)| vec![("/diagram.svg", svg.to_vec())])
        .unwrap_or_default();
    let world = MemWorld::new(source, files);
    let doc = typst::compile::<PagedDocument>(&world)
        .output
        .map_err(|errs| format!("{:?}", errs.iter().map(|e| &e.message).collect::<Vec<_>>()))?;
    let options = PdfOptions {
        standards: PdfStandards::new(&[PdfStandard::Ua_1]).map_err(|e| format!("{e:?}"))?,
        ..PdfOptions::default()
    };
    typst_pdf::pdf(&doc, &options)
        .map_err(|errs| format!("{:?}", errs.iter().map(|e| &e.message).collect::<Vec<_>>()))
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn render(latex: Vec<String>) -> Result<Vec<u8>, String> {
    let refs: Vec<&str> = latex.iter().map(String::as_str).collect();
    render_pdf(&refs, None)
}
