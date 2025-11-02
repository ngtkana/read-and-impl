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

## 解説

### 用語

- 赤黒木 (red-black tree, rbtree): この記事では 2-node、左傾 3-node のみを許したものを赤黒木と呼ぶ
- 前赤黒木 (pre-red-black tree, prbtree): 【この記事の独自定義】赤黒木の部分木となりうる木。

なお prbtree であることは、それ自身が rbtree であるか，もしくは 1 つの rbtree を赤頂点で join したものであることと同値である。
prbtree の根を黒く塗ることで rbtree になることにも注意。

また対応する 2-3 木の node を B-tree node と呼ぶ。


### 不変条件

操作の前後で、(2-node, 左傾 3-node のみからなる) rbtree の条件を満たす。

### 回転

B-tree node 内の辺（いわゆる赤い辺）しか回転しないものとし、回転関数の中で色の付け替えもするものとする。
コードでいうと `x`, `y` が同一の B-tree node に入っている、つまり `y.color == Red` を課していることになる。

なお本実装では記述のために生ポインタを使用しているが、（親ポインタなしの実装のため）実際には循環参照が存在しないので `Box` で実装することもできる。

```rust
unsafe fn rotate_left(x: &mut Node) -> &mut Node {
    let y = &mut *x.right;
    x.right = y.left;
    y.left = x;
    y.color = x.color;
    x.color = Red;
    y
}
unsafe fn rotate_right(x: &mut Node) -> &mut Node {
    let y = &mut *x.left;
    x.left = y.right;
    y.right = x;
    y.color = x.color;
    x.color = Red;
    y
}
```

### 色変更

4-node の分割と 2-node 同士の併合を関数化しておく。

```rust
unsafe fn split_four_node(root: &mut Node) {
    root.color = Red;
    (*root.left).color = Black;
    (*root.right).color = Black;
}
unsafe fn join_two_nodes(root: &mut Node) {
    root.color = Black;
    (*root.left).color = Red;
    (*root.right).color = Red;
}
```


### Insert

行きがけには何もせず、葉に頂点を雑に挿入する。
するともともとの B-tree node の構造と、今挿入した方向により、次の 4 通りの状況がありうる。

| もともと| 追加方向 | 結果 |
| - | - | - |
| 2-node | 左 | 左傾 3-node |
| 2-node | 右 | 右傾 3-node |
| 左傾 3-node | 左 | 左傾 4-node |
| 左傾 3-node | 左 | balanced 4-node |


帰りがけに次の関数 $\mathtt{fixUp}$ を適用していけば、これら全てに対応して修正されていく。
最後の $\mathtt{splitFourNode}$ (定義省略) は親 B-tree node に赤頂点を 1 つ押し付けるが、これは親 B-tree node 内の $\mathtt{fixUp}$ 呼び出しにより解決する。

一連の$\mathtt{fixUp}$ が終了しても依然、根が赤い

```rust
unsafe fn fixup(mut x: &mut Node) -> &mut Node {
    if color(x.right) == Red {
        x = rotate_left(x);
    }
    if color(x.left) == Red && color((*x.left).left) == Red {
        x = rotate_right(x);
    }
    if color(x.left) == Red && color(x.right) == Red {
        split_four_node(x);
    }
    x
}
```

全体的には次のようになる。キーが重複している場合は挿入しない

```rust
unsafe fn insert(mut x: *mut Node, key: i64) -> *mut Node {
    x = insert_recurse(x, key);
    (*x).color = Black;
    x
}
unsafe fn insert_recurse(x: *mut Node, key: i64) -> *mut Node {
    let Some(x) = x.as_mut() else {
        return Box::leak(Box::new(Node {
            left: null_mut(),
            right: null_mut(),
            key,
            color: Red,
        }));
    };
    match key.cmp(&x.key) {
        Ordering::Less => x.left = insert_recurse(x.left, key),
        Ordering::Equal => return x,
        Ordering::Greater => x.right = insert_recurse(x.right, key),
    }
    fixup(x)
}
```


### Remove

行きがけに処理をすることで、次の不変条件を保つ。（4-node は splay していることもあるが、それも含めてここでは lean していると呼んでいる。）

- 現在見ている頂点 $x$ が 3,4-node のいずれかであり、少なくとも $x$ の部分までは $x$ に向かって lean してる
- 根から $x$ までのパス上の頂点がすべて 2,3,4-node のどれかであり、赤頂点がすべて $x$ に向かって lean している

まず根が 2-node だと不変条件の初期条件を満たさないので、その場合は根を赤く塗り、自分自身を3-node の赤頂点であると信じ込ませる。

再帰中は、次のように次の段階に不変条件を遺伝させる:

- 行きたい方向とは逆が赤頂点ならば行きたい方向に lean しておく
- これにより行きたい方向が同一赤頂点になる、もしくは 3-node の黒頂点になればばもう変形しなくて良い
- 問題は行きたい方向が 2-node の黒頂点のときで、そのときは次の関数 $\mathtt{moveRedLeft}$, $\mathtt{moveRedRight}$ を用いてなんとかする。

この関数はぱっと見で分かりづらいが、要するに隣から赤頂点を奪えるなら奪ってダメなら親から$\mathtt{joinTwoNodes}$ で奪ってくるということ。
ここで親からの強奪に成功することは不変条件からわかる。

```rust
unsafe fn move_red_left(mut x: &mut Node) -> &mut Node {
    join_two_nodes(x);
    if color((*x.right).left) == Red {
        x.right = rotate_right(&mut *x.right);
        x = rotate_left(x);
        split_four_node(x);
    }
    x
}
unsafe fn move_red_right(mut x: &mut Node) -> &mut Node {
    join_two_nodes(x);
    if color((*x.left).left) == Red {
        x = rotate_right(x);
        split_four_node(x);
    }
    x
}
```

さて、削除対象までたどり着いたあとの話をする。
まず葉木ではないので中間節点の削除を求められる可能性があるが、その説明は普通の二分探索木と同じなので省略する。

帰りがけの木の修復は、次のパターンに対応できる必要があるが、どのパターンも先述の $\mathtt{fixUp}$ でカバーできる。

- 削除対象の方に lean した 4-node
- 削除対象の方に lean した 3-node
- 2-node

ちなみに削除対象の方に lean していない 4-node は直せないことがあるので注意。（$4 ⋅ C_3 = 20$ パターン全部確かめてみよう！）

```rust
unsafe fn remove(x: *mut Node, key: i64) -> *mut Node {
    let Some(x) = x.as_mut() else {
        return null_mut();
    };
    if color(x.left) == Black {
        x.color = Red;
    }
    let x = remove_recurse(&mut *x, key);
    if let Some(x) = x.as_mut() {
        x.color = Black;
    }
    x
}
unsafe fn remove_recurse(mut x: &mut Node, key: i64) -> *mut Node {
    match key.cmp(&x.key) {
        Ordering::Less => {
            let Some(l) = x.left.as_mut() else {
                return x;
            };
            if l.color == Black && color(l.left) == Black {
                x = move_red_left(x);
            }
            x.left = remove_recurse(&mut *x.left, key);
        }
        Ordering::Equal | Ordering::Greater => {
            if color(x.left) == Red {
                x = rotate_right(x);
            }
            let Some(r) = x.right.as_mut() else {
                return match key.cmp(&x.key) {
                    Ordering::Less => unreachable!(),
                    Ordering::Equal => null_mut(),
                    Ordering::Greater => x,
                };
            };
            if r.color == Black && color(r.left) == Black {
                x = move_red_right(x);
            }
            if x.key == key {
                let removed;
                (x.right, removed) = remove_min(&mut *x.right);
                x.key = (*removed).key;
            } else if !x.right.is_null() {
                x.right = remove_recurse(&mut *x.right, key);
            }
        }
    }
    fixup(x)
}
unsafe fn remove_min(mut x: &mut Node) -> (*mut Node, *mut Node) {
    if x.left.is_null() {
        return (null_mut(), x);
    }
    if color(x.left) == Black && color((*x.left).left) == Black {
        x = move_red_left(x);
    }
    let removed;
    (x.left, removed) = remove_min(&mut *x.left);
    (fixup(x), removed)
}
```

