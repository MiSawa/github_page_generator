---

title: 有理数同士の大小比較
date: 2015/11/26
tags: [有理数]
secret: false
description: 有理数の大小比較をオーバーフローしないように実装する手法について。

---
<div style="display:none">$$
\gdef{\intpart}#1{\lfloor\,#1\,\rfloor}
\gdef{\properpart}#1{\left\{\,#1\,\right\}}
$$</div>

# 有理数同士の大小比較

JAG の ICPC 模擬地区予選 2015 の最中に, ジャッジルームで思いついたこと.

## イントロ

誤差を避けるために有理数を使うとき, オーバーフローするボトルネックは, 多くの場合, 有理数同士の大小比較になる.

(逆に, 有理数同士の大小比較がボトルネックにならないときは, 別のテクニックで誤差を避けられることがある)

そこで, 有理数の分母分子自体はオーバーフローしていないときに, 有理数同士の比較を, なるべくオーバーフローさせずに行う方法を見る.

つまり, 次のような `sgn` 関数を作りたい.

```cpp
template<class Int> struct Fraction{//{{{
    Int num, den;

    private:
    static Int gcd(Int a, Int b){
        while(b) swap(a %= b, b);
        return a;
    }
    void normalize(){
        Int g = gcd(num, den);
        num /= g; den /= g;
        if(den < 0){ num = -num; den = -den; }
    }
    public:
    Fraction(Int num) : num(num), den(1) { }
    Fraction(Int num, Int den, bool need_normalize = true) : num(num), den(den) {
        if(need_normalize) normalize();
    }

    Fraction operator-() const {
        return Fraction(-num, den, false);
    }
};//}}}

using Z = int64_t;
using Q = Fraction<Z>;

int sgn(const Z a){
    return (a > 0) - (a < 0);
}
int sgn(const Z a, const Z b){
    return (a > b) - (a < b); // sgn(a-b) にするとオーバーフローしやすい.
}
int sgn(const Q &a){
    return sgn(a.num);
}
int sgn(const Q &a, const Q &b){
    return sgn(a.num * b.den, b.num * a.den); // オーバーフローしやすい.
}
```


## 分母が小さい場合

`$M$` を, 格納出来る符号付き整数の最大値より少し小さめの値とする. (例えば, `$M = 2^{63} - 1000$`)

分母が `$\sqrt{M}$` 以下な場合を考えよう.
このような有理数は, 例えば点と直線の距離のように, 分子にくる数のオーダーが分母に来る数の二乗になるような場面で出てくる.

この場合については, よく知られたテクニックがある.

`$x$` に対し, `$\intpart{x}$` と `$\properpart{x}$` を, それぞれ `$x$` の整数部分, 小数部分とする.

すると, `$\frac{a}{b}$` と `$\frac{c}{d}$` の大小比較をするには,
`$\left( \intpart{\frac{a}{b}}, \properpart{\frac{a}{b}} \right)$`
と
`$\left( \intpart{\frac{c}{d}}, \properpart{\frac{c}{d}} \right)$`
の辞書順の大小比較をすればよい.

`$\intpart{\frac{a}{b}}$` は, `$a/b$` を切り捨てで求めればよいだけ. (C++ などでは負の場合の丸め方向に注意)

したがって, `$\intpart{\frac{a}{b}}$` と `$\intpart{\frac{c}{d}}$` が違う場合は簡単だから, `$\properpart{\frac{a}{b}}$` と `$\properpart{\frac{c}{d}}$` を比較することを考えればよい.

`$\properpart{\frac{a}{b}}$` は, 悪名高き帯分数を思い出せば,
`$$
\frac{a}{b} = \intpart{\frac{a}{b}} + \frac{a \bmod b}{b}
$$`
だから,
`$$
\properpart{\frac{a}{b}} = \frac{a \bmod b}{b}
$$`
である. ここで, `$a \bmod b$` は `$0 \le (a \bmod b) < b$` となるように取る.

したがって, `$0 \le \text{分子} < \text{分母} \le \sqrt{M}$` であるような二つの有理数の比較に帰着でき, これは単に `$ad$` と `$bc$` を比較すればよい.

```cpp
int sgn(Q a, Q b){
    if(a.num <= 0 or b.num <= 0){
        if(a.num == 0 or b.num == 0) return sgn(a.num, b.num);
        if((a.num < 0) xor (b.num < 0)) return sgn(a.num, b.num);
        return sgn(-b, -a);
    }
    if(int s = sgn(a.num / a.den, b.num / b.den)) return s;
    a.num %= a.den;
    b.num %= b.den;
    return sgn(a.num * b.den, a.den * b.num);
}
```


## 分母が大きい場合

分母が `$\sqrt{M}$` より大きい場合, 先ほどのようなテクニックは使えない.
これを打開しよう.

`$\properpart{\frac{a}{b}} = \frac{a'}{b}$`, `$\properpart{\frac{c}{d}} = \frac{c'}{d}$` とする. (元が既約分数なら, 約分が要らないことに注意)

`$a' = 0$` または `$c' = 0$` のときは, すぐにわかる.
そうでない場合, `$\frac{a'}{b}$` と `$\frac{c'}{d}$` の大小関係は, `$\frac{d}{c'}$` と `$\frac{b}{a'}$` の大小関係と一致する.

これを用いて再帰すれば, いつか `$a' = 0$` か `$c' = 0$` になり, 終了する.

連分数展開を知っているひとは, 有理数の連分数展開を行なっていると思ったほうがわかりやすいかもしれない.

```cpp
int sgn(Q a, Q b){
    if(a.num <= 0 or b.num <= 0){
        if(a.num == 0 or b.num == 0) return sgn(a.num, b.num);
        if((a.num < 0) xor (b.num < 0)) return sgn(a.num, b.num);
        return sgn(-b, -a);
    }
    if(int s = sgn(a.num / a.den, b.num / b.den)) return s;
    a.num %= a.den;
    b.num %= b.den;
    // 0 な方の .den が 1 だから, オーバーフローしない.
    if(a.num == 0 or b.num == 0) return sgn(a.num * b.den, a.den * b.num);
    return sgn(Q(b.den, b.num, false), Q(a.den, a.num, false));
}
// (符号を変える以外)非再帰版
int sgn(Q a, Q b){
    if(a.num <= 0 or b.num <= 0){
        if(a.num == 0 or b.num == 0) return sgn(a.num, b.num);
        if((a.num < 0) xor (b.num < 0)) return sgn(a.num, b.num);
        return sgn(-b, -a);
    }
    int flip = 1;
    while(a.den != 0 and b.den != 0){
        if(int s = sgn(a.num / a.den, b.num / b.den)) return s * flip;
        swap(a.num %= a.den, a.den);
        swap(b.num %= b.den, b.den);
        flip = -flip;
    }
    return flip * sgn(a.num * b.den, a.den * b.num);
}
```


## 計算量

引数が `$(a, b), (c, d)$` から `$(d, c \bmod d), (b, a \bmod b)$` に変わっている.
これは, `$(a, b)$` と `$(c,d)$` に, 同時にユークリッドの互除法をしながら, 商の大小関係を見ているのと同じで, `$O(\log M)$` で出来る.


