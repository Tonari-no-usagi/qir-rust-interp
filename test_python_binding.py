import sys
import os

# ビルドされた共有ライブラリのパスを sys.path に追加
# Windows の場合は .pyd ファイル、Linux/macOS の場合は .so ファイルを探す
target_dir = os.path.join(os.getcwd(), "target", "debug")
sys.path.append(target_dir)

try:
    import qir_rust_interp
    print("Successfully imported qir_rust_interp module.")

    ll_file = os.path.join("samples", "bell.ll")
    if not os.path.exists(ll_file):
        print(f"Error: {ll_file} not found.")
        sys.exit(1)

    print(f"Running simulation for {ll_file}...")
    results = qir_rust_interp.run_qir(ll_file, qubits=10)

    print("\nSimulation Results (Probability Distribution):")
    for bit_string, prob in sorted(results.items()):
        print(f"|{bit_string}>: {prob:.4f}")

except ImportError as e:
    print(f"ImportError: {e}")
    print("\nAvailable files in target/debug:")
    if os.path.exists(target_dir):
        for f in os.listdir(target_dir):
            if f.endswith((".pyd", ".so", ".dll")):
                print(f"  {f}")
except Exception as e:
    print(f"An error occurred: {e}")
