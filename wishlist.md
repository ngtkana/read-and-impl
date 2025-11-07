# wishlist

## BBST の高速化。

- 問題: [Hash Swapping](https://atcoder.jp/contests/soundhound2018-summer-final-open/tasks/soundhound2018_summer_final_e) 
- 過去の速い提出: [非再帰赤黒木 2478 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/47444867)
- 新ライブラリで頑張って散った提出: [再帰AVL木 4006 ms](https://atcoder.jp/contests/soundhound2018-summer-final-open/submissions/70741746)

大きな違いは平衡アルゴリズムと親ポインタの有無であり、このいずれかが実行速度に効いていると考えられる。
まずは [Reversible AVL Tree (Box 実装)](/reversible-avltree-by-box/index.md) と対応して非再帰版を作り、速度を比較してみたい。

