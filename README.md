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
lmemo allview              # エイリアス: all, la
lmemo allview -s <domain>  # 特定のドメインだけ絞り込み
```

### 直近10件を表示

```bash
lmemo view        # エイリアス: v, ls
```

### 日別の追加数をグラフ表示

```bash
lmemo chart       # エイリアス: c
```

### ドメインごとの登録件数を表示

```bash
lmemo site-view   # エイリアス: sv
```

### IDで指定した記事をブラウザで開く

```bash
lmemo open <ID>   # エイリアス: o
```

`view` や `allview` で確認したIDを渡すと、既定のブラウザでそのURLを開きます。

### シェル補完スクリプトを生成

```bash
lmemo completion <shell>   # zsh, bash, fish など
```

例 (zsh):
```bash
lmemo completion zsh > ~/.zfunc/_lmemo
```

## 技術スタック

- Rust (edition 2024)
- [clap](https://docs.rs/clap) — CLIパーサ
- [clap_complete](https://docs.rs/clap_complete) — シェル補完生成
- [rusqlite](https://docs.rs/rusqlite) — SQLite
- [chrono](https://docs.rs/chrono) — 日付
- [ureq](https://docs.rs/ureq) — HTTPクライアント
- [regex](https://docs.rs/regex) — タイトル抽出
- [dirs](https://docs.rs/dirs) — DB保存先のパス解決
- [open](https://docs.rs/open) — OS既定アプリでURLを開く
