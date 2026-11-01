"""Spike PRE-5: protótipo de conversor MathML -> OMML (subconjunto) e DOCX mínimo.

Uso: python3 -I mml2omml.py mathml.tsv saida.docx
"""

import sys
import zipfile
import xml.etree.ElementTree as ET
from xml.sax.saxutils import escape

M = 'xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math"'
NARY = {"∑", "∫", "∏", "∮", "⋃", "⋂"}
FENCES = {"(": ")", "[": "]", "{": "", "|": "|"}


def run(text, plain=False):
    rpr = "<m:rPr><m:sty m:val=\"p\"/></m:rPr>" if plain else ""
    return f"<m:r>{rpr}<m:t xml:space=\"preserve\">{escape(text)}</m:t></m:r>"


def e(inner):
    return f"<m:e>{inner}</m:e>"


def seq(children):
    """Converte uma sequência, agrupando parênteses e n-ários."""
    out, i = [], 0
    kids = list(children)
    while i < len(kids):
        k = kids[i]
        tag = k.tag
        # Operador n-ário com limites: munderover/msubsup cuja base é ∑/∫, seguido do corpo.
        if tag in ("munderover", "msubsup", "munder", "msub", "msup", "mover") and k[0].tag == "mo" and (k[0].text or "").strip() in NARY:
            sym = k[0].text.strip()
            sub = conv(k[1]) if len(k) > 1 and tag not in ("msup", "mover") else ""
            sup = conv(k[-1]) if tag in ("munderover", "msubsup", "msup", "mover") else ""
            body = conv(kids[i + 1]) if i + 1 < len(kids) else ""
            limloc = "undOvr" if tag.startswith("mu") or tag == "mover" else "subSup"
            out.append(
                f"<m:nary><m:naryPr><m:chr m:val=\"{escape(sym)}\"/><m:limLoc m:val=\"{limloc}\"/>"
                f"{'<m:subHide m:val=\"1\"/>' if not sub else ''}{'<m:supHide m:val=\"1\"/>' if not sup else ''}"
                f"</m:naryPr><m:sub>{sub}</m:sub><m:sup>{sup}</m:sup>{e(body)}</m:nary>"
            )
            i += 2
            continue
        out.append(conv(k))
        i += 1
    return "".join(out)


def conv(n):
    tag = n.tag
    txt = (n.text or "").strip()
    if tag in ("math", "mrow", "mstyle", "mpadded", "mphantom", "semantics", "mtd"):
        kids = list(n)
        # mrow delimitado: ( ... ) / { ... } -> m:d
        if tag == "mrow" and len(kids) >= 2 and kids[0].tag == "mo" and (kids[0].text or "").strip() in FENCES and kids[0].get("stretchy") != "false":
            beg = kids[0].text.strip()
            end = kids[-1].text.strip() if kids[-1].tag == "mo" and (kids[-1].text or "").strip() == FENCES[beg] else ""
            inner = kids[1:-1] if end else kids[1:]
            return (f"<m:d><m:dPr><m:begChr m:val=\"{escape(beg)}\"/><m:endChr m:val=\"{escape(end)}\"/></m:dPr>"
                    f"{e(seq(inner))}</m:d>")
        return seq(kids)
    if tag in ("mi", "mn"):
        plain = tag == "mn" or n.get("mathvariant") == "normal" or len(txt) > 1
        return run(txt, plain)
    if tag == "mo":
        return "" if txt == "\u2061" else run(txt, True)
    if tag == "mtext":
        return f"<m:r><m:rPr><m:nor/></m:rPr><m:t xml:space=\"preserve\">{escape(n.text or '')}</m:t></m:r>"
    if tag == "mspace":
        return run(" ", True)
    if tag == "mfrac":
        return f"<m:f>{'<m:fPr><m:type m:val=\"lin\"/></m:fPr>' if n.get('bevelled') else ''}<m:num>{conv(n[0])}</m:num><m:den>{conv(n[1])}</m:den></m:f>"
    if tag == "msup":
        return f"<m:sSup>{e(conv(n[0]))}<m:sup>{conv(n[1])}</m:sup></m:sSup>"
    if tag == "msub":
        return f"<m:sSub>{e(conv(n[0]))}<m:sub>{conv(n[1])}</m:sub></m:sSub>"
    if tag == "msubsup":
        return f"<m:sSubSup>{e(conv(n[0]))}<m:sub>{conv(n[1])}</m:sub><m:sup>{conv(n[2])}</m:sup></m:sSubSup>"
    if tag == "msqrt":
        return f"<m:rad><m:radPr><m:degHide m:val=\"1\"/></m:radPr><m:deg/>{e(seq(n))}</m:rad>"
    if tag == "mroot":
        return f"<m:rad><m:deg>{conv(n[1])}</m:deg>{e(conv(n[0]))}</m:rad>"
    if tag == "mover":
        acc = (n[1].text or "").strip() if n[1].tag == "mo" else ""
        if acc and n.get("accent") == "true":
            return f"<m:acc><m:accPr><m:chr m:val=\"{escape(acc)}\"/></m:accPr>{e(conv(n[0]))}</m:acc>"
        return f"<m:limUpp>{e(conv(n[0]))}<m:lim>{conv(n[1])}</m:lim></m:limUpp>"
    if tag == "munder":
        return f"<m:limLow>{e(conv(n[0]))}<m:lim>{conv(n[1])}</m:lim></m:limLow>"
    if tag == "munderover":
        return (f"<m:limUpp>{e(f'<m:limLow>{e(conv(n[0]))}<m:lim>{conv(n[1])}</m:lim></m:limLow>')}"
                f"<m:lim>{conv(n[2])}</m:lim></m:limUpp>")
    if tag == "mtable":
        rows = "".join("<m:mr>" + "".join(e(conv(td)) for td in tr) + "</m:mr>" for tr in n)
        return f"<m:m>{rows}</m:m>"
    raise ValueError(f"elemento MathML não suportado: <{tag}>")


def strip_ns(root):
    for el in root.iter():
        el.tag = el.tag.split("}")[-1]
    return root


def main(tsv, out):
    paras, failures = [], 0
    for line in open(tsv, encoding="utf-8"):
        status, latex, mathml = line.rstrip("\n").split("\t", 2)
        try:
            omml = conv(strip_ns(ET.fromstring(mathml.replace("&ApplyFunction;", "\u2061"))))
            paras.append(f"<w:p><w:r><w:t xml:space=\"preserve\">{escape(latex)}  →  </w:t></w:r><m:oMath>{omml}</m:oMath></w:p>")
        except Exception as ex:  # noqa: BLE001 — spike: registrar e seguir
            failures += 1
            paras.append(f"<w:p><w:r><w:t>FALHOU {escape(latex)}: {escape(str(ex))}</w:t></w:r></w:p>")
    doc = (f'<?xml version="1.0" encoding="UTF-8" standalone="yes"?>'
           f'<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main" {M}>'
           f'<w:body>{"".join(paras)}</w:body></w:document>')
    with zipfile.ZipFile(out, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("[Content_Types].xml",
                   '<?xml version="1.0" encoding="UTF-8"?><Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">'
                   '<Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>'
                   '<Default Extension="xml" ContentType="application/xml"/>'
                   '<Override PartName="/word/document.xml" ContentType="application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml"/></Types>')
        z.writestr("_rels/.rels",
                   '<?xml version="1.0" encoding="UTF-8"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">'
                   '<Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="word/document.xml"/></Relationships>')
        z.writestr("word/document.xml", doc)
    print(f"{len(paras) - failures}/{len(paras)} fórmulas convertidas; falhas: {failures}")


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
