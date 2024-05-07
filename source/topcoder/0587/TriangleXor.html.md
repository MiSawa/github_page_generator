---

problem:
    round: SRM587
    level: [Div1 Med 550]
    rd: "15699"
    pm: "12528"
    name: TriangleXor
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12528&rd=15699
date: 2015/10/07
tags: [TopCoder, 幾何]

---

### 概要

$ (0, 0),\ (W, 0),\ (W, 1),\ (0, 1) $ を頂点とする $ W \times 1 $ の長方形を考える.

この中に, $ (0, 0),\ (0, 1),\ (x, 0) $ を頂点とする三角形と, $ (W, 0),\ (W, 1),\ (x, 0) $ を頂点とする三角形を, 各 $ 0 < x < W $ に対して書く.

三角形たちの XOR になっている領域の面積の整数部分を答えよ.


#### 制約

$ 1 \le W \le 70{,}000 $.
面積と, 最も近い整数の差は $0.01$ 以上ある.

### 解法

長方形の対角線で分断された四つの領域について考える.

上の領域は, $ W $ の偶奇に従って, $ W / 4 $ か $ 0 $ のいずれか.

左右の領域は $ O(W) $ でわかる.

下のメッシュ状の領域は, 大体 $ W / 8 $ くらい.

最初の数ケース(サンプルにある)が AC だったし, $ W $ が増えるほど精度良くなりそうだから, これで提出したら AC だった.

ちゃんと求める方法もあるようだ.

### ソースコード

~~~ cpp
// 342.57 pts

int TriangleXor::theArea( int W ){
    using R = long double;
    R res = 0;
    {
        R prev = 0;
        for(int x = 1; x <= W; ++x){
            R curr = R(1) / (1 / R(W) + 1 / R(x)) / 2;
            if(x % 2) res += curr - prev;
            prev = curr;
        }
        res *= 2;
        if(W % 2 == 0) res += W / R(4);
    }
    res += R(W) / 8;

    return res;
}
~~~

