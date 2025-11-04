# Reversible AVL Tree

[reversible-left-leaning-rbtree](../../reversible-left-leaning-rbtree/README.md) の AVL 木版。

実行速度はほぼ同じで、実装はこちらのほうがずっと易しい。

## 解説

$\mathrm{merge2}$, $\mathrm{split2}$ さえ作ればあとは完全に同じなので、そこまで解説する。


### 回転

同じ。


### balance

LLRB でいう $\mathrm{fixup}$ の役割。

- 子を回転する前には $\mathrm{push}$ が必要。
- この関数は未 $\mathrm{update}$ の状態でも呼んで良いことにしたいので、rebalance しないときも $\mathrm{update}$ だけは行う

```rust
unsafe fn balance(mut x: &mut Node) -> &mut Node {
    match ht(x.left) as i8 - ht(x.right) as i8 {
        -2 => {
            if let Some(r) = x.right.as_mut().filter(|r| ht(r.left) < ht(r.right)) {
                r.push();
                x.right = rotate_left(r);
            }
            x = rotate_left(x);
        }
        -1..=1 => x.update(),
        2 => {
            if let Some(l) = x.left.as_mut().filter(|l| ht(l.left) < ht(l.right)) {
                l.push();
                x.left = rotate_left(l);
            }
            x = rotate_right(x);
        }
        _ => unreachable!(),
    }
    x
}
```

### merge3

潜りながら $\mathrm{push}$ を行い、帰りに $\mathrm{balance}$ を行う。

```rust
unsafe fn merge3(l: *mut Node, c: &mut Node, r: *mut Node) -> &mut Node {
    match height(l).cmp(&height(r)) {
        Ordering::Less => {
            (*r).push();
            (*r).left = merge3(l, c, (*r).left);
            balance(&mut *r)
        }
        Ordering::Equal => {
            c.left = l;
            c.right = r;
            c.update();
            c
        }
        Ordering::Greater => {
            (*l).push();
            (*l).right = merge3((*l).right, c, r);
            balance(&mut *l)
        }
    }
}
```

### split3

潜りながら $\mathrm{push}$ を行う。$\mathrm{balance}$ は $\mathrm{merge3}$ が担ってくれているので不要。

```rust
unsafe fn split3(x: &mut Node, index: usize) -> (*mut Node, &mut Node, *mut Node) {
    x.push();
    let llen = x.left.as_ref().map_or(0, |l| l.len);
    let l = mem::replace(&mut x.left, null_mut());
    let r = mem::replace(&mut x.right, null_mut());
    match index.cmp(&llen) {
        Ordering::Less => {
            let (ll, lc, lr) = split3(&mut *l, index);
            (ll, lc, ptr::from_mut(merge3(lr, x, r)))
        }
        Ordering::Equal => {
            x.update();
            (l, x, r)
        }
        Ordering::Greater => {
            let (rl, rc, rr) = split3(&mut *r, index - 1 - llen);
            (ptr::from_mut(merge3(l, x, rl)), rc, rr)
        }
    }
}
```

### merge2/split2

LLRB と異なり根の色などを木にする必要がない。

```rust
unsafe fn split2(x: *mut Node, index: usize) -> (*mut Node, *mut Node) {
    let Some(indexm1) = index.checked_sub(1) else {
        return (null_mut(), x);
    };
    let (l, c, r) = split3(&mut *x, indexm1);
    (ptr::from_mut(merge3(l, c, null_mut())), r)
}
unsafe fn merge2(l: *mut Node, r: *mut Node) -> *mut Node {
    let Some(r) = r.as_mut() else { return l };
    let (_, c, r) = split3(r, 0);
    merge3(l, c, r)
}
```


## 実行速度検証

[提出 3166 ms](https://judge.yosupo.jp/submission/326021)

