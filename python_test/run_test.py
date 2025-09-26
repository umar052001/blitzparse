import time

import blitz_parse

# Path to the complex sample file
docx_path = "tests/assets/sample.docx"

print(f"--- Calling Rust to parse: {docx_path} ---")

# --- Performance Timing ---
start_time = time.time()

try:
    # This is the call to your high-performance Rust function!
    extracted_text = blitz_parse.extract_text_py(docx_path)

    end_time = time.time()
    # --- End Timing ---

    print(f"\n✅ Success! Extraction complete in {end_time - start_time:.4f} seconds.")
    print("\n--- Start of Extracted Text ---")
    # Print the first 500 characters to keep the output clean
    print(extracted_text[:500] + "...")
    print("--- End of Extracted Text ---")

except ValueError as e:
    print(f"\n❌ An error occurred: {e}")
