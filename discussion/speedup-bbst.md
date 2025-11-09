# BBST の高速化について

- 問題: [Hash Swapping](https://atcoder.jp/contests/soundhound2018-summer-final-open/tasks/soundhound2018_summer_final_e) 
- 過去の速い提出: [非再帰赤黒木 2478 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/47444867)
- 新ライブラリ (`avl-tree`) で頑張って散った提出: [再帰AVL木 4006 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/70741746)

大きな違いは

1. 平衡アルゴリズムと
2. 親ポインタの有無、
3. そして遅延伝搬機能(主にrev)の有無

あり、このいずれかが実行速度に効いていると考えられる。
まずは [Reversible AVL Tree (Box 実装)](/reversible-avltree-by-box/README.md) と対応して非再帰版を作り、速度を比較してみたい。

ちなみにライブラリを用いて Dynamic Sequence Range Affine Range Sum に挑戦しても [3135 ms](https://judge.yosupo.jp/submission/327248) と、抽象化前と区別不可能なくらいの違いしか無いため、抽象化のせいではない

## 検証

### 遅延伝播機能の有無 → 変化無し

機能を消したバージョンを作って Hash Swapping に提出してみたが、[4138 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/70742332) とほぼ変わらなかった。

### 非再帰化（`Vec`）→ より悪化

再起する代わりに `Vec` でコールスタックの代わりになるものを作ってみたが、実行速度はより悪化した。

[5338 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/70836013)

```rust
pub fn merge3<C: NodeMarker>(
    mut l: Option<Box<Node<C>>>,
    mut c: Box<Node<C>>,
    mut r: Option<Box<Node<C>>>,
) -> Box<Node<C>> {
    let mut stack = vec![];
    loop {
        match ht(l.as_deref()).cmp(&ht(r.as_deref())) {
            Ordering::Less => {
                let mut e = r.unwrap();
                e.push();
                r = e.left.take();
                stack.push(MergeStackEntry::Right(e));
            }
            Ordering::Equal => {
                c.left = l;
                c.right = r;
                c.update();
                break;
            }
            Ordering::Greater => {
                let mut e = l.unwrap();
                e.push();
                l = e.right.take();
                stack.push(MergeStackEntry::Left(e));
            }
        }
    }
    while let Some(e) = stack.pop() {
        match e {
            MergeStackEntry::Left(mut l) => {
                l.right = Some(c);
                c = balance(l);
            }
            MergeStackEntry::Right(mut r) => {
                r.left = Some(c);
                c = balance(r);
            }
        }
    }
    c
}

enum MergeStackEntry<C: NodeMarker> {
    Left(Box<Node<C>>),
    Right(Box<Node<C>>),
}

pub fn split3<C: NodeMarker>(
    mut x: Box<Node<C>>,
    mut index: usize,
) -> (Option<Box<Node<C>>>, Box<Node<C>>, Option<Box<Node<C>>>) {
    let mut stackl = vec![];
    let mut stackr = vec![];
    let (mut l0, x0, mut r0) = loop {
        x.push();
        let llen = x.left.as_ref().map_or(0, |l| l.len);
        let l = x.left.take();
        let r = x.right.take();
        match index.cmp(&llen) {
            Ordering::Less => {
                stackr.push((x, r));
                x = l.unwrap();
            }
            Ordering::Equal => {
                x.update();
                break (l, x, r);
            }
            Ordering::Greater => {
                stackl.push((l, x));
                x = r.unwrap();
                index -= llen + 1;
            }
        }
    };
    while let Some((l, x)) = stackl.pop() {
        l0 = Some(merge3(l, x, l0));
    }
    while let Some((x, r)) = stackr.pop() {
        r0 = Some(merge3(r0, x, r));
    }
    (l0, x0, r0)
}
```
