import shutil
from typing import Optional
import urllib.parse
import markdown
from markdown.extensions import Extension
from markdown.preprocessors import Preprocessor
from pymdownx.superfences import SuperFencesException
import re
import pathlib
import subprocess
import urllib
from hashlib import sha224

__all__ = ["MyExtension"]

PREVIEW_START_REGEX = re.compile(
    r"(?P<fence>~{3,}|`{3,})[ \t]*(?P<lang>typst(?:-code)?)[ \t]+.*(?P<preview>\+preview(?:\((?P<options>[^\)]*)\))?)"
)
# example{
# example(Title){
EXAMPLE_START_REGEX = re.compile(r"^example\s*(?:\((?P<title>[^)]+)\))?\s*{$")
NESTED_FENCE_END = r"%s%s[ \t]*\n?$"  # from superfences
PREFIX_CHARS = (">", " ", "\t")  # from superfences
INDENTED_LIST = r"^%s\d+"
WHITESPACE_RE = re.compile(r"^[ \t]*\n?$")

docs_dir: Optional[str] = None
site_dir: Optional[str] = None
site_path: Optional[str] = None


class PreviewPreprocessor(Preprocessor):
    ext: "MyExtension"

    def __init__(self, ext: "MyExtension"):
        """Initialize."""
        self.ext = ext

    def parse_whitespace(self, line: str):
        """Parse the whitespace"""
        ws = []
        for c in line:
            if c not in PREFIX_CHARS:
                break
            ws.append(c)

        return "".join(ws)

    def run(self, lines):
        transformed = []

        i = 0
        in_example = False
        while i < len(lines):
            line = lines[i]

            ws = self.parse_whitespace(line)

            # Found the start of a fenced block.
            m = PREVIEW_START_REGEX.match(line, len(ws))
            if m is None:
                m = EXAMPLE_START_REGEX.match(line.strip())
                if m is not None:
                    line = f'<details class="example" markdown="1"><summary>{m.group("title") or "Example"}</summary>'
                    in_example = True
                elif in_example and (line.strip() == "}" or line.strip() == "}example"):
                    line = "</details>"
                    in_example = False
                transformed.append(line)
                i += 1
                continue

            lang = m.group("lang")
            preview = m.group("preview")
            options = (m.group("options") or "").replace(",", "")

            def take_opt(name: str) -> bool:
                nonlocal options
                if name in options:
                    options = options.replace(name, "")
                    return True
                return False

            classes = ""
            if take_opt("vertical"):
                classes += "vertical"
                options.replace("vertical", "")
            if take_opt("01-to-02"):
                lang = "typst-01-to-02"
            options = f'{options} lang="{lang}"'
            transformed.append(
                ws
                + f'<div class="typst-preview {classes}" markdown="1"><div class="typst-source" markdown="1">'
            )
            transformed.append(line.replace(preview, ""))

            fence = m.group("fence")
            fence_end = re.compile(NESTED_FENCE_END % (ws, fence))

            # find the end of this fence
            block = []
            k = i
            for k in range(i + 1, len(lines)):
                line = lines[k]
                block.append(line)
                transformed.append(line)
                if fence_end.match(line) is None:
                    continue
                break

            # check for annotation lists
            last_code = k + 1
            last_list = -1
            indent_re = re.compile(INDENTED_LIST % ws)
            for k in range(last_code, len(lines)):
                line = lines[k]
                if WHITESPACE_RE.match(line):
                    continue
                if indent_re.match(line):
                    last_list = k
                else:
                    break

            # put the annotations in (this doesn't support multiline lists yet)
            if last_list != -1:
                transformed.extend(lines[last_code : last_list + 1])
                transformed.append("")
                i = last_list + 1
            else:
                i = last_code

            transformed.append(ws + '</div><div class="preview">')
            transformed.append(ws + fence + "typst-preview " + options)
            transformed.extend(block)
            transformed.append(ws + "</div></div>")

        return transformed


class MyExtension(Extension):
    def __init__(self, config, **kwargs):
        global docs_dir
        if "docs_dir" in config:
            docs_dir = config["docs_dir"]
        global site_dir
        if "site_dir" in config:
            site_dir = config["site_dir"]
        global site_path
        if "site_url" in config:
            site_path = urllib.parse.urlparse(config["site_url"]).path
            assert site_path is not None
            if not site_path.endswith("/"):
                site_path += "/"

        Extension.__init__(self, **kwargs)

    def extendMarkdown(self, md):
        md.preprocessors.register(PreviewPreprocessor(self), "typst-preview", 40)


TEMPLATE_HEADER = r"""
#let _is-dark = %s
#set page(width: auto, height: auto, margin: 0.75cm, fill: none)
#let _accent = if _is-dark { white } else { black }
#set text(fill: _accent)
#set table(stroke: _accent.transparentize(30%%))

"""

# requires `just deploy`
IMPORTS = r"""
#import "@local/icu-datetime:0.2.1" as icu
"""
IMPORTS_01_TO_02 = r"""
#import "@preview/icu-datetime:0.2.0" as icu02
#import "@preview/icu-datetime:0.1.2" as icu01
"""
TEMPLATES = {
    "code": TEMPLATE_HEADER
    + IMPORTS
    + r"""
    #{
        %s
    }
    """,
    "embedded": TEMPLATE_HEADER + IMPORTS + "%s",
    "01-to-02": TEMPLATE_HEADER + IMPORTS_01_TO_02 + "%s",
    "basic": TEMPLATE_HEADER + "%s",
}


def custom_formatter(
    source: str,
    language: str,
    css_class: str,
    options: dict[str, str],
    md: markdown.Markdown,
    classes=None,
    id_value="",
    attrs=None,
    **kwargs,
):
    try:
        if "mode" not in options:
            raise ValueError("Missing 'mode'")
        if options["mode"] not in TEMPLATES:
            raise ValueError(f"Invalid mode: {options['mode']}")
        assert docs_dir and site_dir and site_path

        def render(is_dark: bool):
            src = source
            if options["fake"]:
                src = re.sub(r"^#import.+", "", src)
            return TEMPLATES[options["mode"]] % ("true" if is_dark else "false", src)

        light_doc = render(False)
        dark_doc = render(True)
        base = sha224(bytes(light_doc, "UTF-8"), usedforsecurity=False).hexdigest()
        dump_folder = pathlib.Path(docs_dir) / "rendered"
        if not dump_folder.exists():
            dump_folder.mkdir(parents=True)
        site_folder = pathlib.Path(site_dir) / "rendered"
        if not site_folder.exists():
            site_folder.mkdir(parents=True)

        def paths_for(is_dark: bool):
            filename = f"{base}_{'d' if is_dark else 'l'}.svg"
            return (
                dump_folder / filename,
                site_folder / filename,
                f"{site_path}rendered/{filename}",
            )

        light_dump_path, lite_site_path, lite_web_path = paths_for(False)
        dark_dump_path, dark_site_path, dark_web_path = paths_for(True)
        if not light_dump_path.exists() or not dark_dump_path.exists():
            render_typst([(light_doc, light_dump_path), (dark_doc, dark_dump_path)])
            if site_dir is not None:
                shutil.copy2(light_dump_path, lite_site_path)
                shutil.copy2(dark_dump_path, dark_site_path)
    except Exception as e:
        raise SuperFencesException from e

    return f'<img src="{lite_web_path}" alt="Preview" loading="lazy"><img src="{dark_web_path}" alt="Preview" loading="lazy">'


def custom_validator(
    language: str,
    inputs: dict[str, str],
    options: dict[str, str],
    attrs: dict[str, str],
    md: markdown.Markdown,
):
    """Custom validator."""
    if "mode" in inputs:
        mode = inputs["mode"]
    elif "lang" in inputs:
        match inputs["lang"]:
            case "typst-code":
                mode = "code"
            case "typst":
                mode = "embedded"
            case "typst-01-to-02":
                mode = "01-to-02"
            case _:
                raise ValueError(f"Invalid language: {inputs['lang']}")
    else:
        mode = "embedded"
    options["mode"] = mode
    options["fake"] = "fake" in inputs
    return True


def get_docs_dir():
    global docs_dir
    if docs_dir is not None:
        return docs_dir
    return str((pathlib.Path(__file__).parent.parent / "docs").resolve())


def render_typst_doc(arg: tuple[str, pathlib.Path]):
    code, path = arg
    p = subprocess.run(
        ["typst", "c", "-", path],
        check=False,
        capture_output=True,
        text=True,
        input=code,
    )
    if p.returncode != 0:
        print(p.stdout)
        print(p.stderr)
        print(f"------\ntrying to compile:\n------\n{code}")
        raise RuntimeError(f"typst exited with {p.returncode}")


def render_typst(docs: list[tuple[str, pathlib.Path]]):
    # TODO: do this in parallel
    for doc in docs:
        render_typst_doc(doc)
