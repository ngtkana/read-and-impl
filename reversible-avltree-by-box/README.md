# Reversible AVL Tree (Box 実装)

[reversible-avltree](../../reversible-avltree/README.md) において、生ポインタの代わりに `Box` を使ったもの。

`unsafe` が必要なくなり、`unsafe_op_in_unsafe_fn` も破らなくて済むようになった。
その代わり `clippy::unnecessary_box_returns` を破っているが、これは pedantic なのと、そもそもずっと `Box` で取り回しているため `Box` のままのほうがよいと判断して抑制している。

実行速度は依然として殆ど変わらない。


## 解説

### 回転

所有権の都合上植え替えの際には常に `take` と move を使わないといけないため、更新順に注意。

```rust
fn rotate_left(mut x: Box<Node>) -> Box<Node> {
    x.push();
    let mut y = x.right.take().unwrap();
    y.push();
    x.right = y.left.take();
    x.update();
    y.left = Some(x);
    y.update();
    y
}
fn rotate_right(mut x: Box<Node>) -> Box<Node> {
    x.push();
    let mut y = x.left.take().unwrap();
    y.push();
    x.left = y.right.take();
    x.update();
    y.right = Some(x);
    y.update();
    y
}
```


### balance

私の Rust 力の問題かもしれないが、rebalance 条件が少し書きづらくなった。
`ht` は `Option<&Node>` を受け取るようにしたので呼ぶときには `as_deref` が必要。

```rust
fn balance(mut x: Box<Node>) -> Box<Node> {
    match ht(x.left.as_deref()) as i8 - ht(x.right.as_deref()) as i8 {
        -2 => {
            x.right = x.right.map(|mut r| {
                if ht(r.left.as_deref()) > ht(r.right.as_deref()) {
                    r.push();
                    rotate_right(r)
                } else {
                    r
                }
            });
            x = rotate_left(x);
        }
        -1..=1 => x.update(),
        2 => {
            x.left = x.left.map(|mut l| {
                if ht(l.left.as_deref()) < ht(l.right.as_deref()) {
                    l.push();
                    rotate_left(l)
                } else {
                    l
                }
            });
            x = rotate_right(x);
        }
        _ => unreachable!(),
    }
    x
}
```

### merge/split

ほとんど変わらないが若干書きやすい気がしないでもない。
`mem::replace` を使わなくて良くなった点は少し読みやすい気がする。

```rust
fn merge2(l: Option<Box<Node>>, mut r: Option<Box<Node>>) -> Option<Box<Node>> {
    let Some(r) = r.take() else { return l };
    let (_, c, r) = split3(r, 0);
    Some(merge3(l, c, r))
}
fn merge3(l: Option<Box<Node>>, mut c: Box<Node>, r: Option<Box<Node>>) -> Box<Node> {
    match ht(l.as_deref()).cmp(&ht(r.as_deref())) {
        Ordering::Less => {
            let mut r = r.unwrap();
            r.push();
            r.left = Some(merge3(l, c, r.left));
            balance(r)
        }
        Ordering::Equal => {
            c.left = l;
            c.right = r;
            c.update();
            c
        }
        Ordering::Greater => {
            let mut l = l.unwrap();
            l.push();
            l.right = Some(merge3(l.right, c, r));
            balance(l)
        }
    }
}
fn split2(x: Option<Box<Node>>, index: usize) -> (Option<Box<Node>>, Option<Box<Node>>) {
    if index == 0 {
        return (None, x);
    }
    let (l, c, r) = split3(x.unwrap(), index - 1);
    (Some(merge3(l, c, None)), r)
}
fn split3(mut x: Box<Node>, index: usize) -> (Option<Box<Node>>, Box<Node>, Option<Box<Node>>) {
    x.push();
    let llen = x.left.as_ref().map_or(0, |l| l.len);
    let l = x.left.take();
    let r = x.right.take();
    match index.cmp(&llen) {
        Ordering::Less => {
            let (ll, lc, lr) = split3(l.unwrap(), index);
            (ll, lc, Some(merge3(lr, x, r)))
        }
        Ordering::Equal => {
            x.update();
            (l, x, r)
        }
        Ordering::Greater => {
            let (rl, rc, rr) = split3(r.unwrap(), index - 1 - llen);
            (Some(merge3(l, x, rl)), rc, rr)
        }
    }
}
```

## 実行速度検証

[提出 3152 ms](https://judge.yosupo.jp/submission/326041)

