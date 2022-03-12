# make-pitagoe-db

[\[Japanease Edition\]](./README.ja.md)

Create a csv file and caracter.ini file for [ぴた声](https://www.ah-soft.com/pitagoe/index.html).

## Usage

Run `make-pitagoe-db.exe` on the command line.

```
make-pitagoe-db.exe --name <Name of software talker>> --path <Pass to register for ぴた声>
```

The current specification searches for files with the extension `wav` under the path to be registered.

If the execution is successful, `<directory name>.csv` and `character.ini` will be created in the path to register ぴた声.

## License

MIT or Apache 2.0
