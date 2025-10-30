# Left-Leaning Red-Black Trees (Sedgewick)

Robert Sedgewick 氏による左傾赤黒木の実装のRust 移植

## 参考文献

* [Swdgewick 氏の Web ページ](https://sedgewick.io/)
* [ペーパー (10 pages)](https://sedgewick.io/wp-content/themes/sedgewick/papers/2008LLRB.pdf): コードが最も完全に近い？
* [トークスライド 1 (78 pages)](https://sedgewick.io/wp-content/uploads/2022/03/2008-09LLRB.pdf): 2-3 木版。Delete は
* [別のトークスライド 2 (64 pages)](https://pdfs.semanticscholar.org/7cfb/8f56cabd723eb0b2a69f8ad3d0827ebc2f4b.pdf): 2-3-4 木版。Delete のスライドは目次にだけ合って触れられていない。（？）

## わかったこと・わからなかったこと

* 操作前後に保つべき不変条件は明確。
    * 根が黒、黒高さ平衡
    * 2-3 木版なら、2-node or left-leaning 3-node
    * 2-3-4 木版なら、2-node, left-leaning 3-node, or left-leaning 4-node
* ペーパー中に実装のなかった `fixUp()` 関数は、説明から `insert()` の最後の 3 つの操作を続けたものだと理解した。
* 根付近の場合分け。これは上に挙げた文献に於いても不完全なはず。
    * `delete()` を行うとき、根が 2-node だった場合はループ不変条件の初期条件を満たさないはず。
    * 2-3-4 木版の場合は `insert()` 時に根が 4-node になるとループ不変条件の初期条件を満たさないはず。


## 実装の方針

* ペーパーの実装をなるべくそのまま写経する。
* 根付近の場合分けは推測で実装する。
* インターフェースは、`default()`, `insert()`, `remove()` のみとし、後者ふたつは `bool` を返すものとする。（注：原典では値を返していない）
* Rust に書き換える都合上、読みやすさのために少し書き換えるが、なるべく原典との同値性がわかりやすい書き方を心がける。


## テスト

* `insert()`, `remove()` を一定の確率分布で呼んで、その結果を `std::collections::HashSet` と比較する。
* 呼び出しの後に毎回不変条件をチェックする。


## 実行速度検証

### 問題

[ABC 303 C - Dash](https://atcoder.jp/contests/abc303/tasks/abc303_c)

### クエリ回数評価

* `insert()`: $4 \times 10^5$ 回以下
* `remove()`: $2 \times 10^5$ 回以下

### 実行時間

| ライブラリ | 実行時間 | 提出 |
| - | - | - |
| `HashSet` | 26 ms | [https://atcoder.jp/contests/abc303/submissions/70558004]() |
| `BTreeSet` | 35 ms | [https://atcoder.jp/contests/abc303/submissions/70558088]() |
| `Rbtree` | 111 ms | [https://atcoder.jp/contests/abc303/submissions/70558077]() |

