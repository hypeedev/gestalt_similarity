# gestalt_similarity
Gestalt similarity extension for SQLite3

## Building from source
```
cargo build --release
```

## Installation and usage

SQLite CLI:

```
sqlite> .load ./path/to/libgestalt_similarity.so
sqlite> SELECT SIMILARITY('duck', 'quack');
```

Node.js, using [better-sqlite3](https://github.com/WiseLibs/better-sqlite3):

```js
import Database from "better-sqlite3";

const db = new Database(":memory:");
db.loadExtension("./path/to/libgestalt_similarity.so");

const similarity = db.prepare("SELECT SIMILARITY(?, ?)").get("duck", "quack");

db.close();
```

Python:

```python
import sqlite3

conn = sqlite3.connect(":memory:")
conn.enable_load_extension(True)
conn.load_extension("./path/to/libgestalt_similarity.so")
cursor = conn.cursor()

cursor.execute("SELECT SIMILARITY(?, ?)", ("duck", "quack"))

similarity = cursor.fetchone()[0]

cursor.close()
conn.close()
```
