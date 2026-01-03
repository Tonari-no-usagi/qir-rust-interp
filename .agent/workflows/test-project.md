---
description: プロジェクトの動作テスト（Rust CLI & Pythonバインディング）を実行する
---

このワークフローでは、Rust製のCLIインタプリタとPythonバインディングの両方が正しく動作することを確認します。

### 1. プロジェクトのビルド
以下のコマンドを実行して、Rustライブラリとバイナリをビルドします。
// turbo
```powershell
cargo build
```

### 2. Pythonバインディングの準備 (Windows)
ビルドされた `.dll` を Python が認識できる `.pyd` 拡張子にコピーします。
// turbo
```powershell
copy target\debug\qir_rust_interp.dll target\debug\qir_rust_interp.pyd
```

### 3. Rust CLI のテスト
サンプルファイルを実行して、確率分布が表示されるか確認します。
// turbo
```powershell
cargo run -- samples/bell.ll
cargo run -- samples/teleportation.ll
```

### 4. Python バインディングのテスト
Python スクリプトを実行して、モジュールのインポートと実行ができるか確認します。
// turbo
```powershell
python test_python_binding.py
```
