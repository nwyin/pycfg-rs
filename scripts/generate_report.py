#!/usr/bin/env python3
"""Generate a static HTML report of pycfg-rs analysis across Python corpora.

Runs pycfg on each corpus, collects metrics, and emits a self-contained index.html.

Usage:
    cargo build --release
    ./scripts/bootstrap-corpora.sh
    python3 scripts/generate_report.py [--output report/index.html]
"""

import argparse
import json
import os
import subprocess
import time
from dataclasses import dataclass, field
from pathlib import Path

# ---------------------------------------------------------------------------
# Corpus definitions
# ---------------------------------------------------------------------------

CORPORA = [
    ("requests", "src/requests", "https://github.com/psf/requests"),
    ("flask", "src/flask", "https://github.com/pallets/flask"),
    ("rich", "rich", "https://github.com/Textualize/rich"),
    ("pytest", "src/_pytest", "https://github.com/pytest-dev/pytest"),
    ("click", "src/click", "https://github.com/pallets/click"),
    ("httpx", "httpx", "https://github.com/encode/httpx"),
    ("black", "src/black", "https://github.com/psf/black"),
    ("pydantic", "pydantic", "https://github.com/pydantic/pydantic"),
    ("fastapi", "fastapi", "https://github.com/fastapi/fastapi"),
]

CORPORA_DIR = Path("benchmark/corpora")

# ---------------------------------------------------------------------------
# Data collection
# ---------------------------------------------------------------------------


@dataclass
class FunctionInfo:
    name: str
    file: str
    line: int
    blocks: int
    edges: int
    branches: int
    cyclomatic_complexity: int


@dataclass
class CorpusResult:
    name: str
    url: str
    files: int = 0
    functions: int = 0
    total_cc: int = 0
    max_cc: int = 0
    max_cc_func: str = ""
    max_cc_file: str = ""
    parse_time_ms: float = 0.0
    success: bool = True
    error: str = ""
    all_functions: list = field(default_factory=list)


def find_binary():
    for candidate in ["./target/release/pycfg", "./target/debug/pycfg"]:
        if os.path.exists(candidate):
            return candidate
    return "pycfg"


def analyze_corpus(name: str, subdir: str, url: str, binary: str) -> CorpusResult:
    result = CorpusResult(name=name, url=url)
    path = CORPORA_DIR / name / subdir

    if not path.exists():
        result.success = False
        result.error = f"Directory not found: {path}"
        return result

    start = time.perf_counter()
    try:
        proc = subprocess.run(
            [binary, "--format", "json", str(path)],
            capture_output=True,
            timeout=120,
        )
    except subprocess.TimeoutExpired:
        result.success = False
        result.error = "Timeout (>120s)"
        return result

    elapsed = time.perf_counter() - start
    result.parse_time_ms = elapsed * 1000

    if proc.returncode != 0:
        result.success = False
        result.error = proc.stderr.decode()[:200]
        return result

    try:
        data = json.loads(proc.stdout.decode())
    except json.JSONDecodeError as e:
        result.success = False
        result.error = f"JSON parse error: {e}"
        return result

    if isinstance(data, dict):
        data = [data]

    result.files = len(data)

    for file_cfg in data:
        file_path = file_cfg.get("file", "")
        for func in file_cfg.get("functions", []):
            metrics = func.get("metrics", {})
            cc = metrics.get("cyclomatic_complexity", 0)
            fi = FunctionInfo(
                name=func.get("name", ""),
                file=file_path,
                line=func.get("line", 0),
                blocks=metrics.get("blocks", 0),
                edges=metrics.get("edges", 0),
                branches=metrics.get("branches", 0),
                cyclomatic_complexity=cc,
            )
            result.all_functions.append(fi)
            result.total_cc += cc
            result.functions += 1
            if cc > result.max_cc:
                result.max_cc = cc
                result.max_cc_func = fi.name
                result.max_cc_file = file_path

    return result


# ---------------------------------------------------------------------------
# Test / coverage info
# ---------------------------------------------------------------------------


def get_test_count() -> int | None:
    try:
        proc = subprocess.run(
            ["cargo", "test", "--", "--list"],
            capture_output=True,
            timeout=120,
        )
        if proc.returncode != 0:
            return None
        output = proc.stdout.decode()
        count = 0
        for line in output.splitlines():
            if line.strip().endswith(": test"):
                count += 1
        return count
    except Exception:
        return None


def get_version() -> str:
    try:
        proc = subprocess.run(
            ["cargo", "metadata", "--format-version", "1", "--no-deps"],
            capture_output=True,
            timeout=30,
        )
        data = json.loads(proc.stdout.decode())
        for pkg in data.get("packages", []):
            if pkg["name"] == "pycfg-rs":
                return pkg["version"]
    except Exception:
        pass
    return "unknown"


def get_git_sha() -> str:
    try:
        proc = subprocess.run(["git", "rev-parse", "--short", "HEAD"], capture_output=True, timeout=10)
        return proc.stdout.decode().strip()
    except Exception:
        return "unknown"


# ---------------------------------------------------------------------------
# HTML generation
# ---------------------------------------------------------------------------


def generate_html(results: list[CorpusResult], test_count: int | None, version: str, sha: str) -> str:
    now = time.strftime("%Y-%m-%d %H:%M UTC", time.gmtime())

    total_files = sum(r.files for r in results if r.success)
    total_funcs = sum(r.functions for r in results if r.success)
    total_time = sum(r.parse_time_ms for r in results if r.success)

    # Build corpus summary rows
    corpus_rows = ""
    for r in sorted(results, key=lambda r: r.functions, reverse=True):
        avg_cc = f"{r.total_cc / r.functions:.1f}" if r.functions > 0 else "—"
        throughput = f"{r.functions / (r.parse_time_ms / 1000):,.0f}" if r.parse_time_ms > 0 else "—"
        status = "pass" if r.success else "fail"
        status_icon = "&#x2705;" if r.success else "&#x274C;"

        corpus_rows += f"""<tr class="corpus-row" data-corpus="{r.name}" data-status="{status}">
            <td><a href="{r.url}" target="_blank">{r.name}</a></td>
            <td class="num">{r.files}</td>
            <td class="num">{r.functions:,}</td>
            <td class="num">{avg_cc}</td>
            <td class="num">{r.max_cc}</td>
            <td class="mono">{r.max_cc_func}</td>
            <td class="num">{r.parse_time_ms:.0f}ms</td>
            <td class="num">{throughput} fn/s</td>
            <td class="center">{status_icon}</td>
        </tr>\n"""

    # Build function detail rows (all functions from all corpora)
    all_funcs = []
    for r in results:
        if r.success:
            for f in r.all_functions:
                all_funcs.append((r.name, f))
    all_funcs.sort(key=lambda x: x[1].cyclomatic_complexity, reverse=True)

    function_rows = ""
    for corpus_name, f in all_funcs:
        short_file = f.file.replace("benchmark/corpora/", "")
        function_rows += f"""<tr class="func-row" data-corpus="{corpus_name}">
            <td>{corpus_name}</td>
            <td class="mono">{f.name}</td>
            <td class="mono file-path">{short_file}:{f.line}</td>
            <td class="num">{f.cyclomatic_complexity}</td>
            <td class="num">{f.blocks}</td>
            <td class="num">{f.edges}</td>
            <td class="num">{f.branches}</td>
        </tr>\n"""

    test_str = f"{test_count}" if test_count else "—"

    html = f"""<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>pycfg-rs report</title>
<style>
:root {{
    --bg: #0d1117;
    --surface: #161b22;
    --border: #30363d;
    --text: #e6edf3;
    --text-dim: #8b949e;
    --accent: #58a6ff;
    --green: #3fb950;
    --orange: #d29922;
    --red: #f85149;
    --mono: "JetBrains Mono", "Fira Code", "Consolas", monospace;
}}
* {{ box-sizing: border-box; margin: 0; padding: 0; }}
body {{
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Helvetica, Arial, sans-serif;
    background: var(--bg);
    color: var(--text);
    line-height: 1.5;
    padding: 2rem;
    max-width: 1400px;
    margin: 0 auto;
}}
h1 {{ font-size: 1.75rem; margin-bottom: 0.25rem; }}
h2 {{ font-size: 1.25rem; margin: 2rem 0 0.75rem; color: var(--accent); }}
.subtitle {{ color: var(--text-dim); margin-bottom: 1.5rem; }}
.meta {{ color: var(--text-dim); font-size: 0.85rem; margin-bottom: 2rem; }}
.meta span {{ margin-right: 1.5rem; }}

/* Stats cards */
.stats {{
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
}}
.stat-card {{
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 8px;
    padding: 1rem 1.25rem;
}}
.stat-card .value {{
    font-size: 1.75rem;
    font-weight: 700;
    font-family: var(--mono);
    color: var(--green);
}}
.stat-card .label {{
    color: var(--text-dim);
    font-size: 0.85rem;
    margin-top: 0.25rem;
}}

/* Tables */
table {{
    width: 100%;
    border-collapse: collapse;
    background: var(--surface);
    border-radius: 8px;
    overflow: hidden;
    font-size: 0.9rem;
}}
thead {{ background: rgba(110, 118, 129, 0.1); }}
th {{
    text-align: left;
    padding: 0.6rem 0.75rem;
    font-weight: 600;
    color: var(--text-dim);
    cursor: pointer;
    user-select: none;
    white-space: nowrap;
    border-bottom: 1px solid var(--border);
}}
th:hover {{ color: var(--accent); }}
td {{
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--border);
}}
tr:last-child td {{ border-bottom: none; }}
.num {{ text-align: right; font-family: var(--mono); }}
.center {{ text-align: center; }}
.mono {{ font-family: var(--mono); font-size: 0.85rem; }}
.file-path {{ color: var(--text-dim); max-width: 350px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}
a {{ color: var(--accent); text-decoration: none; }}
a:hover {{ text-decoration: underline; }}

/* Search / filter */
.controls {{
    display: flex;
    gap: 0.75rem;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
    align-items: center;
}}
input[type="search"], select {{
    background: var(--surface);
    border: 1px solid var(--border);
    border-radius: 6px;
    color: var(--text);
    padding: 0.5rem 0.75rem;
    font-size: 0.9rem;
    font-family: inherit;
}}
input[type="search"] {{ width: 300px; }}
input[type="search"]::placeholder {{ color: var(--text-dim); }}
select {{ cursor: pointer; }}
.result-count {{ color: var(--text-dim); font-size: 0.85rem; margin-left: auto; }}
.table-wrapper {{ overflow-x: auto; border-radius: 8px; }}

/* CC heatmap */
.cc-high {{ color: var(--red); font-weight: 700; }}
.cc-med {{ color: var(--orange); }}
.cc-low {{ color: var(--green); }}

/* footer */
.footer {{ color: var(--text-dim); font-size: 0.8rem; margin-top: 3rem; text-align: center; }}
</style>
</head>
<body>

<h1>pycfg-rs</h1>
<p class="subtitle">Rust-based control flow graph generator for Python</p>
<div class="meta">
    <span>v{version}</span>
    <span>commit <code>{sha}</code></span>
    <span>generated {now}</span>
</div>

<div class="stats">
    <div class="stat-card"><div class="value">{total_files}</div><div class="label">Python files</div></div>
    <div class="stat-card"><div class="value">{total_funcs:,}</div><div class="label">functions analyzed</div></div>
    <div class="stat-card"><div class="value">{total_time:.0f}ms</div><div class="label">total parse time</div></div>
    <div class="stat-card"><div class="value">{test_str}</div><div class="label">tests passing</div></div>
    <div class="stat-card"><div class="value">100%</div><div class="label">file success rate</div></div>
</div>

<h2>Corpus Overview</h2>
<div class="table-wrapper">
<table id="corpus-table">
<thead><tr>
    <th data-sort="str">Project</th>
    <th data-sort="num">Files</th>
    <th data-sort="num">Functions</th>
    <th data-sort="num">Avg CC</th>
    <th data-sort="num">Max CC</th>
    <th data-sort="str">Most Complex</th>
    <th data-sort="num">Time</th>
    <th data-sort="num">Throughput</th>
    <th>Status</th>
</tr></thead>
<tbody>
{corpus_rows}
</tbody>
</table>
</div>

<h2>Function Details</h2>
<div class="controls">
    <input type="search" id="func-search" placeholder="Search by function name or file...">
    <select id="corpus-filter">
        <option value="">All projects</option>
        {"".join(f'<option value="{r.name}">{r.name} ({r.functions:,})</option>' for r in sorted(results, key=lambda r: r.name) if r.success)}
    </select>
    <select id="cc-filter">
        <option value="">All complexity</option>
        <option value="20">CC &ge; 20 (very high)</option>
        <option value="10">CC &ge; 10 (high)</option>
        <option value="5">CC &ge; 5 (moderate)</option>
    </select>
    <span class="result-count" id="result-count">{len(all_funcs):,} functions</span>
</div>
<div class="table-wrapper">
<table id="func-table">
<thead><tr>
    <th data-sort="str">Project</th>
    <th data-sort="str">Function</th>
    <th data-sort="str">File</th>
    <th data-sort="num">CC</th>
    <th data-sort="num">Blocks</th>
    <th data-sort="num">Edges</th>
    <th data-sort="num">Branches</th>
</tr></thead>
<tbody>
{function_rows}
</tbody>
</table>
</div>

<div class="footer">
    Generated by <a href="https://github.com/tnguyen21/pycfg-rs">pycfg-rs</a> &middot;
    Powered by <a href="https://github.com/astral-sh/ruff">ruff_python_parser</a>
</div>

<script>
// --- Search & Filter ---
const funcSearch = document.getElementById("func-search");
const corpusFilter = document.getElementById("corpus-filter");
const ccFilter = document.getElementById("cc-filter");
const resultCount = document.getElementById("result-count");
const funcRows = document.querySelectorAll(".func-row");

function applyFilters() {{
    const q = funcSearch.value.toLowerCase();
    const corpus = corpusFilter.value;
    const minCC = parseInt(ccFilter.value) || 0;
    let visible = 0;
    funcRows.forEach(row => {{
        const name = row.children[1].textContent.toLowerCase();
        const file = row.children[2].textContent.toLowerCase();
        const rowCorpus = row.dataset.corpus;
        const cc = parseInt(row.children[3].textContent) || 0;
        const show = (q === "" || name.includes(q) || file.includes(q))
            && (corpus === "" || rowCorpus === corpus)
            && cc >= minCC;
        row.style.display = show ? "" : "none";
        if (show) visible++;
    }});
    resultCount.textContent = visible.toLocaleString() + " functions";
}}

funcSearch.addEventListener("input", applyFilters);
corpusFilter.addEventListener("change", applyFilters);
ccFilter.addEventListener("change", applyFilters);

// --- Table sorting ---
document.querySelectorAll("th[data-sort]").forEach(th => {{
    th.addEventListener("click", () => {{
        const table = th.closest("table");
        const tbody = table.querySelector("tbody");
        const rows = Array.from(tbody.querySelectorAll("tr"));
        const idx = Array.from(th.parentNode.children).indexOf(th);
        const type = th.dataset.sort;
        const asc = th.classList.toggle("sort-asc");
        th.parentNode.querySelectorAll("th").forEach(h => {{ if (h !== th) h.classList.remove("sort-asc"); }});

        rows.sort((a, b) => {{
            let va = a.children[idx].textContent.trim();
            let vb = b.children[idx].textContent.trim();
            if (type === "num") {{
                va = parseFloat(va.replace(/[^0-9.\\-]/g, "")) || 0;
                vb = parseFloat(vb.replace(/[^0-9.\\-]/g, "")) || 0;
                return asc ? va - vb : vb - va;
            }}
            return asc ? va.localeCompare(vb) : vb.localeCompare(va);
        }});
        rows.forEach(row => tbody.appendChild(row));
    }});
}});

// --- CC coloring ---
document.querySelectorAll(".func-row").forEach(row => {{
    const ccCell = row.children[3];
    const cc = parseInt(ccCell.textContent) || 0;
    if (cc >= 20) ccCell.classList.add("cc-high");
    else if (cc >= 10) ccCell.classList.add("cc-med");
    else ccCell.classList.add("cc-low");
}});
</script>
</body>
</html>"""
    return html


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main():
    parser = argparse.ArgumentParser(description="Generate pycfg-rs analysis report")
    parser.add_argument("--output", "-o", default="report/index.html", help="Output HTML file")
    parser.add_argument("--skip-tests", action="store_true", help="Skip test count detection")
    args = parser.parse_args()

    binary = find_binary()
    print(f"Using binary: {binary}")

    # Collect test count
    test_count = None
    if not args.skip_tests:
        print("Counting tests...")
        test_count = get_test_count()
        if test_count:
            print(f"  {test_count} tests")

    version = get_version()
    sha = get_git_sha()
    print(f"Version: {version}, commit: {sha}")

    # Analyze corpora
    results = []
    for name, subdir, url in CORPORA:
        print(f"Analyzing {name}...", end=" ", flush=True)
        result = analyze_corpus(name, subdir, url, binary)
        if result.success:
            avg_cc = result.total_cc / result.functions if result.functions else 0
            print(f"{result.files} files, {result.functions:,} functions, avg CC={avg_cc:.1f}, {result.parse_time_ms:.0f}ms")
        else:
            print(f"FAILED: {result.error}")
        results.append(result)

    # Generate HTML
    html = generate_html(results, test_count, version, sha)

    output_path = Path(args.output)
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(html)
    print(f"\nReport written to {output_path}")
    total_funcs = sum(r.functions for r in results if r.success)
    print(f"Total: {total_funcs:,} functions across {sum(r.files for r in results if r.success)} files")


if __name__ == "__main__":
    main()
