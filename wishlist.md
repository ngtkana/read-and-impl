# wishlist

## BBST の高速化。

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
