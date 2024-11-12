import pyo3_example
import pandas as pd
import polars
# Create a DataFrame with 3 int32 columns
data = {
    "session_id": pd.Series([1, 2, 3, 4, 5], dtype="int64"),
    "item_id": pd.Series([5, 6, 7, 8, 9], dtype="int64"),
    "timestamp": pd.Series([9, 10, 11, 12, 13], dtype="int64"),
}
df = pd.DataFrame(data)

pdf = polars.DataFrame(df)
print("Python: Original DataFrame:")
print(df)

print('Calling the Rust API from Python')

processed_df = pyo3_example.debug(pdf)

print("\nPolars Dataframe from Rust:")
print(processed_df)
print("\nResult as pandas dataframe:")
print(processed_df.to_pandas())
