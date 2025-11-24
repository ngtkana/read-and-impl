# Bitvector Rank

ビットベクトルの $\mathtt{rank}$ の速度比較。 $w = 64$ とする。

問題サイズ: $N = 10⁸, Q = 10⁷$

## Rank1 ($65n$ bit)

すべての場所の累積和をメモする。 $(1 + w)n$ bit。

- $\mathtt{construct}$: 90 ms
- $\mathtt{rank}$: 45 ms


## Rank64 ($2n$ bit)

- $\mathtt{block}$: $64$ bit ごとのグローバルな累積和 (空間 $n$ bit)

```rust
pub struct Rank64 {
    len: usize,
    words: Vec<u64>,
    block: Vec<u64>,
}
```

- $\mathtt{construct}$: 33 ms
- $\mathtt{rank}$: 44 ms


## Rank25664 ($1.375n$ bit)

- $\mathtt{sblock}$: $256$ bit ごとのグローバルな累積和 (空間 $n / 8$ bit)
- $\mathtt{block}$: $64$ bit ごとの $256$-bit block 内累積和 (空間 $n / 4$ bit)

$256$ ビットごとに累積和をメモする。`block` が $n / 8$ bit、`sblock` が $n / 4$ bit。

```rust
pub struct Rank25664 {
    len: usize,
    words: Vec<u64>,
    block: Vec<u8>,
    sblock: Vec<u64>,
}
```

- $\mathtt{construct}$: 34 ms
- $\mathtt{rank}$: 36 ms



## Rank25664Interlaced ($1.5n$ bit)

$256$ bit (= $4$ word) ごとに、$2$ word のメモを先頭に挟む。

- $\mathtt{words}[6 * a]$: グローバル累積和
- $\mathtt{words}[6 * a + 1]$: $64$ bit ごとの $256$-block 内累積和 $8$ つを、$8$ bit 整数で表して pack したもの
- $\mathtt{words}[6 * a + 2..]$: 生ビットベクトル

```rust
pub struct Rank25664 {
    len: usize,
    words: Vec<u64>,
}
```
- $\mathtt{construct}$: 34 ms
- $\mathtt{rank}$: 32 ms


## Rank51264Interlaced ($1.25n$ bit)

$512$ bit (= $8$ word) ごとに、$2$ word のメモを先頭に挟む。$9$-bit 整数を無理矢理 pack しているので、byte 境界に沿わない read が生じることに注意。

- $\mathtt{words}[10 * a]$: グローバル累積和
- $\mathtt{words}[10 * a + 1]$: $64$ bit ごとの $512$-block 内累積和のうち先頭以外のもの $7$ つを、$9$ bit 整数で表して pack したもの
- $\mathtt{words}[10 * a + 2..]$: 生ビットベクトル

```rust
pub struct Rank25664 {
    len: usize,
    words: Vec<u64>,
}
```
- $\mathtt{construct}$: 33 ms
- $\mathtt{rank}$: 31 ms
