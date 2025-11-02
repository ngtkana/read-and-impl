# Reversible left-Leaning Red-Black Trees

[left-leaning-rbtree](../../left-leaning-rbtree/README.md) の続き。
Library Checker の[Dynamic Sequence Range Affine Range Sum](https://judge.yosupo.jp/problem/dynamic_sequence_range_affine_range_sum)
で verify する都合上、order statistic tree の形で実装する。

## 機能

木の操作としては、次の機能をサポートする。

- $\mathtt{insert}(i, x)$
- $\mathtt{remove}(i)$
- $\mathtt{merge}(T, U)$
- $\mathtt{split}(i)$
- $\mathtt{reverse}(s, t)$

さらに Library Checker で verify するため、次のクエリに対応する。

- $\mathtt{sum}(s, t)$: $[s, t[$ の範囲の総和を答える
- $\mathtt{affine}(s, t, c_1, c_0)$: $[s, t[$ の範囲の要素をそれぞれ $x ↦ c_1 x + c_0$ する。


## 解説

まずは $\mathtt{reverse}$ や値の集約、作用など一切なく、単に $\mathtt{merge}$, $\mathtt{split}$ をサポートした左傾赤黒木を目指す。
その後、それを用いて各種機能を実現していく。

### 3-way merge (join)

$\mathtt{insert}$ とだいたい同じ。
各頂点に黒高さを管理する。頂点の黒高さは、その頂点の**子から**葉までの黒頂点の個数**プラス1**で定義する。

```rust
unsafe fn merge3(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    let root = merge_recurse(l, c, r);
    root.color = Black;
    root
}
unsafe fn merge_recurse(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    let root = match (l.as_ref().map_or(0, |p| p.bh)).cmp(&r.as_ref().map_or(0, |p| p.bh)) {
        Less => {
            let r = push(&mut *r);
            r.left = merge_recurse(l, c, r.left);
            r
        }
        Equal => {
            c.left = l;
            c.right = r;
            c.color = Red;
            c
        }
        Greater => {
            let l = push(&mut *l);
            l.right = merge_recurse(l.right, c, r);
            l
        }
    };
    fixup(root)
}
```

### 3-way split

$\mathtt{remove}$ とだいたい同じだが、削除対象を見つけたときには普通に split すればよいだけな分そこは少し楽。

内部で $\mathtt{mergeRecurse}$ を使っているが、この頂点に渡す木は事前に根を黒くしておかないといけないことに注意。


```rust
unsafe fn split3(mut x: &mut Node, index: usize) -> (*mut Node, &mut Node, *mut Node) {
    if x.color == Black && color(x.left) == Black {
        x.color = Red;
    }
    x = push(x);
    let (l, removed, r) = split_recurse(x, index);
    (l, removed, r)
}
unsafe fn split_recurse(mut x: &mut Node, index: usize) -> (*mut Node, &mut Node, *mut Node) {
    let (l, removed, r) = match index.cmp(&(x.left.as_mut().map_or(0, |p| p.len))) {
        Less => {
            if (*x.left).color == Black && color((*x.left).left) == Black {
                x = move_red_left(x);
                (*x.right).color = Black;
            }
            let (l, removed, r) = split_recurse(&mut *x.left, index);
            let r = merge_recurse(r, x, x.right);
            r.color = Black;
            (l, removed, ptr::from_mut(r))
        }
        Equal => {
            let l = mem::replace(&mut x.left, null_mut());
            let r = mem::replace(&mut x.right, null_mut());
            if let Some(l) = l.as_mut() {
                l.color = Black;
            }
            x.color = Red;
            (l, x, r)
        }
        Greater => {
            if color(x.left) == Red {
                x = rotate_right(x);
            }
            x.right = push(&mut *x.right);
            if (*x.right).color == Black && color((*x.right).left) == Black {
                x = move_red_right(x);
                (*x.left).color = Black;
            }
            let (l, removed, r) = split_recurse(&mut *x.right, index + (*x.right).len - x.len);
            let l = merge_recurse(x.left, x, l);
            l.color = Black;
            (ptr::from_mut(l), removed, r)
        }
    };
    (l, removed, r)
}
```

### merge/split

これは明らか。

```rust
unsafe fn merge2(l: *mut Node, r: *mut Node) -> *mut Node {
    let Some(l) = l.as_mut() else { return r };
    let Some(r) = r.as_mut() else { return l };
    let (_, c, r) = split3(r, 0);
    merge3(l, c, r)
}
unsafe fn split2(x: *mut Node, index: usize) -> (*mut Node, *mut Node) {
    let Some(indexm1) = index.checked_sub(1) else {
        return (null_mut(), x);
    };
    let (l, c, r) = split3(&mut *x, indexm1);
    (merge3(l, c, null_mut()), r)
}
```

## insert/remove

前作ですでに実装済みだが、$\mathtt{merge}$, $\mathtt{split}$ を用いて実装し直すとより簡単になる。


## sum, affine など

$\mathtt{merge}$, $\mathtt{split}$ を用いると全体適用に帰着するので、あとは遅延セグ木と同じようなことをすれば良い。
どこで $\mathtt{push}$, $\mathtt{update}$ を行うべきかは難しい問題であると同時に、説明の難しい問題なので、ソースコードを参照すること。


## reverse

ふつうの平衡二分木（ふつうの赤黒木、AVL 木、splay 木、treap など）を使っていればこれは $\mathtt{affine}$ と同じく、遅延セグ木をやるだけである。
しかし左傾赤黒木には対称性がなく、左傾 3-node の黒頂点に立った $\mathtt{rev}$ フラグを素直に適用してしまうと、それは右傾 3-node となり、不変条件を破ってしまう

そこで本記事では、3-node の黒頂点を $\mathtt{push}$ する前に、その赤い子を $\mathtt{push}$ し、右回転しておくことで対処することとする。
これにより `push` 関数のシグニチャを `&mut Node` ではなく `&mut Node -> &mut Node` にする必要がある。


```rust
fn push(mut x: &mut Node) -> &mut Node {
    unsafe {
        if (x.c1, x.c0) != (Fp::new(1), Fp::new(0)) {
            if let Some(p) = x.left.as_mut() {
                (p.c1, p.c0) = (p.c1 * x.c1, p.c0 * x.c1 + x.c0);
                p.value = p.value * x.c1 + x.c0;
                p.sum = p.sum * x.c1 + x.c0 * Fp::new(p.len as u64);
            }
            if let Some(p) = x.right.as_mut() {
                (p.c1, p.c0) = (p.c1 * x.c1, p.c0 * x.c1 + x.c0);
                p.value = p.value * x.c1 + x.c0;
                p.sum = p.sum * x.c1 + x.c0 * Fp::new(p.len as u64);
            }
            (x.c1, x.c0) = (Fp::new(1), Fp::new(0));
        }
        if x.rev {
            x.rev = false;
            if color(x.left) == Red {
                x.left = push(&mut *x.left);
                x = rotate_right(x);
            }
            mem::swap(&mut x.left, &mut x.right);
            if let Some(p) = x.left.as_mut() {
                p.rev ^= true;
            }
            if let Some(p) = x.right.as_mut() {
                p.rev ^= true;
            }
        }
        x
    }
}
```

この対処法は思いついたばかりなので、実はもっと簡単な対処法がある可能性は高いが、考えるのに疲れたので、通ればよかろうなのだということにしておく。


## 実行速度検証

Dynamic Sequence Range Affine Range Sum

[3198 ms](https://judge.yosupo.jp/submission/325416)
