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
    * Delete を行うとき、根が 2-node だった場合はループ不変条件の初期条件を満たさないはず。
    * 2-3-4 木版の場合は insert 時に根が 4-node になるとループ不変条件の初期条件を満たさないはず。


## 実装の方針

* ペーパーの実装をなるべくそのまま写経する。
* 根付近の場合分けは推測で実装する。
* インターフェースは、`insert()`, `remove()` のみとし、またこれらは値を返さないものとする。（正当性は、内部的な不変条件を確かめることでのみ保証する。）
* Rust に書き換える都合上、読みやすさの観点から同値性を保って局所的な書き換えはする。
