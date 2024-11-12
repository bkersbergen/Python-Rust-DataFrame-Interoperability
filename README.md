# Proof of Concept: Python-Rust DataFrame Interoperability

This project demonstrates a proof of concept for integrating Python with Rust to exchange data using Pandas DataFrames. The code showcases two main features:

1. **Sending a Pandas DataFrame from Python to Rust:** The Python code serializes the DataFrame and sends it to Rust, where it is parsed and processed.
2. **Creating a DataFrame in Rust and sending it to Python:** Rust generates a DataFrame, serializes it, and sends it back to Python for further processing.

## Setup

To get started, follow these steps:

1. **Create a Virtual Environment:**

   It's recommended to create a Python virtual environment to isolate the project dependencies. Run the following commands:

   ```bash
   python310 -m venv venv
   source venv/bin/activate  # On Windows use `venv\Scripts\activate`
   ```
   
2. **Install Python Dependencies:**
   
    Once the virtual environment is activated, install the required Python packages using the `requirements.txt` file:
    ```bash
    pip install -r requirements.txt
    ```

3. **Rust Setup:**
   Ensure that the correct version of Rust is installed on your system.
    ```bash
    rustup update
    ```

## Dependencies

This project relies heavily on specific versions of Python, Rust, and associated libraries. Please ensure the following versions are installed for the code to function correctly:

Python [version 3.10.x] 

Rust [version 1.82.x]

Compatibility issues may arise if versions deviate from those specified.

### Example Output
`
maturin develop --release && python dataframe.py
`
which should produce output like:

```bash
(.venv) maturin develop --release && python dataframe.py
🔗 Found pyo3 bindings
🐍 Found CPython 3.10 at pyo3_example\.venv\Scripts\python.exe
📡 Using build options features from pyproject.toml
    Finished `release` profile [optimized] target(s) in 0.39s
📦 Built wheel for CPython 3.10 to .tmpwBiBoA\pyo3_example-0.1.0-cp310-none-win_amd64.whl
✏️  Setting installed package as editable
🛠 Installed pyo3_example-0.1.0
Python: Original DataFrame:
   session_id  item_id  timestamp
0           1        5          9
1           2        6         10
2           3        7         11
3           4        8         12
4           5        9         13
Calling the Rust API from Python

Polars Dataframe from Rust:
shape: (5, 3)
┌────────────┬─────────┬────────────┐
│ session_id ┆ item_id ┆ timestamp  │
│ ---        ┆ ---     ┆ ---        │
│ u64        ┆ u64     ┆ u64        │
╞════════════╪═════════╪════════════╡
│ 1          ┆ 101     ┆ 1609459200 │
│ 2          ┆ 102     ┆ 1609459260 │
│ 3          ┆ 103     ┆ 1609459320 │
│ 4          ┆ 104     ┆ 1609459380 │
│ 5          ┆ 105     ┆ 1609459440 │
└────────────┴─────────┴────────────┘

Result as pandas dataframe:
   session_id  item_id   timestamp
0           1      101  1609459200
1           2      102  1609459260
2           3      103  1609459320
3           4      104  1609459380
4           5      105  1609459440

```

This output shows the successful transfer of data between Python and Rust using DataFrames, with the DataFrame being passed and transformed between both languages.


#### Troubleshooting
Serious compile errors with "rustc 1.78.0"
Solution: Upgrade to rust "1.82.0" using `rustup update`.


