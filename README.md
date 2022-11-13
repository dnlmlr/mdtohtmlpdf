# Markdown to HTML and PDF

This project aims to generate github style HTML and PDF files from Markdown without LaTeX.
In order to create PDF file, chrome or chromium need to be installed on the system. 

While the whole thing does generally work, the code and architecture is currently so bad that it 
might cause lasting brain damage while reading.

## Installation

```
cargo install --git https://github.com/dnlmlr/mdtohtmlpdf.git
```

## Usage

```
Usage: github-markdown-to-html.exe [OPTIONS] <INPUT>

Arguments:
  <INPUT>  The input Markdown file

Options:
  -o, --html-out <HTML_OUT>  Converter HTML output
  -p, --pdf-out <PDF_OUT>    Converted PDF output
  -h, --help                 Print help information
  -V, --version              Print version information
```

## Example

This whole README can be found in rendered PDF form in the assets directory to view the rendering.

This is a bit of **Markdown** in order to demonstrate the *conversion*. 

- [x] A rust code example
- [ ] Good code

```rust
fn main() {
    println!("Hello world");
}
```

The O-Notation of Hello World is $O(1)$, which is better then $O(n \log{n^{2*2}})$.

$$
X = \sum_{i = 0}^5{i \cdot 2}
$$

This sum results in the following values:
| $i$ | $i \cdot 2$ | $X$ |
|:---:|:-----------:|:---:|
| 0   | 0           | 0   |
| 1   | 2           | 2   |
| 2   | 4           | 6   |
| 3   | 6           | 12  |
| 4   | 8           | 20  |
| 5   | 10          | 30  |

- Abc
  - Cde
    - Efg

1. Abc
   1. Cde
      1. Efg