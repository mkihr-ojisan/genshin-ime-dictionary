# Genshin IME Dictionary

[Genshin Impact Wiki](https://genshin-impact.fandom.com/)のデータを元に原神用語のIME辞書を作成します。

## 使い方
```console
$ cargo run --release > dictionary.txt
```

## 既知の問題
- 長音記号が含まれる単語の読みが不正確なことがある
- すべての単語が固有名詞に分類されている