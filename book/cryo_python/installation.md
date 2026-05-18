# Installation

The `cryo` Python package is not published to PyPI; build it from source with
[maturin](https://github.com/PyO3/maturin).

It requires **Python 3.10 or newer** and a Rust toolchain
([rustup](https://rustup.rs/)), plus a C compiler for cryo's TLS dependencies.

```
pip install maturin
git clone https://github.com/trial123Zel/cryo-ocd
cd cryo-ocd/crates/python
maturin build --release
pip install --force-reinstall <path-printed-by-maturin>.whl
```

During development, run `maturin develop --release` from `crates/python`
instead — it builds the extension and installs it into the active virtual
environment in one step.

## pandas output

cryo returns [polars](https://pola.rs/) DataFrames. To get pandas DataFrames
instead (`output_format="pandas"`), also install the pandas extra:

```
pip install pandas pyarrow
```
