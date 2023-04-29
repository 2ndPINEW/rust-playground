# 学んだことメモ

## Cargo の使い方

### 開発

- `cargo new` を使ってプロジェクトを作成できる
- `cargo build` を使ってプロジェクトをビルドできる
- `cargo run` を使うとプロジェクトのビルドと実行を 1 ステップで行える
- `cargo check` を使うとバイナリを生成せずにプロジェクトをビルドして、エラーがないか確認できる

ビルド成果は`target/debug`に出力される

### リリース

`cargo build --release`で最適化したバイナリが出力される
`target/release`
