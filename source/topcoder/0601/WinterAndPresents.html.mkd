---

problem:
    round: SRM601
    level: [Div1 Easy 250]
    rd: "15713"
    pm: "12860"
    name: WinterAndPresents
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12860&rd=15713
date: 2015/09/30
tags: [TopCoder]

---

### 概要

サイズが $ n \le 50 $ の非負整数配列 $ \mathit{apple} $ と $ \mathit{orange} $ が与えられる.

$ 1 \le X $ を選び, 各 $ i $ について,

$ 0 \le a_i \le \mathit{apple}_i $

と $ 0 \le o_i \le \mathit{orange} $ となる $ a_i, b_i $ を $ a_i + b_i = X $ となるように選ぶ.

組 $ (\sum a_i, \sum b_i) $ の種類として考えれれるものはいくつあるか.

#### 制約

$ 1 \le n \le 50,\ 0 \le \mathit{apple}_i, \mathit{orange}_i \le 10^6 $.

### 解法

$ X $ としてありうるのは, $ 1 $ 以上 $ \min_i \mathit{apple}_i + \mathit{orange}_i $ 以下のすべて.

この各 $ X $ に対し,

0. $ a_i + b_i = X $ だから, 目一杯 $ a_i $ を小さくすると, $ a_i \le \max(0, x - \mathit{orange}_i) $.

0. 従って, $ \sum \mathit{apple}_i $ は, 少なくとも $ \sum \max(0, (x - \mathit{orange}_i)) $.

0. 同様に, $ \sum \mathit{orange}_i $ の下限もわかる.

0. これらは二つの下限は, 実際に達成可能.

0. 一つずつ $ \mathit{++}a_i, \mathit{--}b_i $ のようなことをすれば, その間の全てが達成可能なことがわかる.

あとは足すだけ.

### ソースコード

~~~ cpp
// 180.20
long long WinterAndPresents::getNumber( vector <int> apple, vector <int> orange ){
    long long res = 0;
    const int n = size(apple);
    int m = numeric_limits<int>::max();
    rep(i, n) chmin(m, apple[i] + orange[i]);
    rep(x, m+1){
        int a = 0, o = 0;
        rep(i, n){
            a += max(0, x - orange[i]);
            o += max(0, x -  apple[i]);
        }
        res += (long long)(x) * n - a - o + 1;
    }
    return res - 1;
}
~~~

