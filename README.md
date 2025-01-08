# rust-playground
rust/actics-webでRestAPIを構築.

## 概要
- [オラクルな Dept/Emp テーブル](https://docs.oracle.com/cd/E57425_01/121/LNPCC/GUID-034B6416-444A-4EF1-9078-35F5A86CF1D9.htm) のCRUD/RestAPIを構築.


## 環境構築
### 前提
- 下記インストール済
  - Docker Desktop
  - vscode / DevContainers（拡張機能）

### 手順
1. リポジトリをクローンする
    ```bash
    git clone https://github.com/tamken/rust-playground.git
    ```
2. カレントディレクトリへ移動し、`.env`を作成する
    ```bash
    cd rust-playground
    cp .env.example .env
    ```
3. DevContainerを起動する
    - vscodeの画面左下の`><` をクリックし、開いたメニューから`コンテナで再度開く`を選択する

4. （DevContainer起動後）DBマイグレーションを実行する
    ```bash
    cargo make migrate
    ```

5. APIを起動する（ホットリロード）
    ```bash
    cargo make watch
    ```

## その他
- タスク定義（`cargo-make`）
    |コマンド|概要|
    |:---|:---|
    |cargo make watch|プログラム起動（ホットリロード）|
    |cargo make format|フォーマッター|
    |cargo make lint|リンター|
    |cargo make run|プログラム起動|
    |cargo make migrate|DBマイグレーション実行|
- `settings.json`
  - 保存時に自動フォーマット