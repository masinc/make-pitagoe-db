# make-pitagoe-db

[ぴた声](https://www.ah-soft.com/pitagoe/index.html) 用の csvファイルと caracter.ini ファイルを作成します。

## 使い方

`make-pitagoe-db.exe` をコマンドラインで実行します。

```
make-pitagoe-db.exe --name <ソフトウェアトーク名> --path <ぴた声に登録するパス>
```

現在の仕様では登録するパス以下の拡張子が `wav` ファイルを探索します。

成功した場合、ぴた声に登録するパスに `<ディレクトリ名>.csv` と `character.ini` が作成されます。

## ライセンス

MIT or Apache 2.0
