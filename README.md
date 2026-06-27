# learning-memo

学んだURLをコマンドラインから記録・閲覧できる、SQLite ベースのシンプルなCLIツール。

## インストール

```bash
git clone https://github.com/iorinu/learning-memo.git
cd learning-memo
cargo install --path .
```

`~/.cargo/bin` にバイナリ `lmemo` がインストールされます。

## 使い方

### 学習リストに追加

```bash
lmemo add <URL> [--title <タイトル>] [--memo <メモ>]
```

`--title` を省略するとURLからページタイトルを自動取得します。

例:
```bash
lmemo add https://example.com -m "Rustの公式チュートリアル"
```

### 全件表示

```bash
lmemo allview     # エイリアス: all, la
```

### 直近10件を表示

```bash
lmemo view        # エイリアス: v, ls
```

### 日別の追加数をグラフ表示

```bash
lmemo chart       # エイリアス: c
```

## 技術スタック

- Rust (edition 2024)
- [clap](https://docs.rs/clap) — CLIパーサ
- [rusqlite](https://docs.rs/rusqlite) — SQLite
- [chrono](https://docs.rs/chrono) — 日付
- [ureq](https://docs.rs/ureq) — HTTPクライアント
- [regex](https://docs.rs/regex) — タイトル抽出
