# Bitvector Rank

ビットベクトルの $\mathrm{rank}$ の速度比較。 $w = 64$ とする。

問題サイズ: $N = 10⁸, Q = 10⁷$

## Rank1 ($65n$ bit)

すべての場所の累積和をメモする。 $(1 + w)n$ bit。

- $\mathrm{construct}$: ~91 ms
- $\mathrm{rank}$: ~45 ms


## Rank64 ($2n$ bit)

$64$ ビットごとに累積和をメモする。ブロック内の $\mathrm{rank}$ は $\mathrm{count\\_ones}$ を用いる。

```rust
pub struct Rank64 {
    len: usize,
    words: Vec<u64>,
    block: Vec<u64>,
}
```

- $\mathrm{construct}$: ~34 ms
- $\mathrm{rank}$: ~35 ms


## Rank25664 ($1.375n$ bit)

$256$ ビットごとに累積和をメモする。`block` が $n / 8$ bit、`sblock` が $n / 4$ bit。

```rust
pub struct Rank25664 {
    len: usize,
    words: Vec<u64>,
    block: Vec<u8>,
    sblock: Vec<u64>,
}
```

- 構築: 33 ms
- $\mathrm{rank}$: 35ms
