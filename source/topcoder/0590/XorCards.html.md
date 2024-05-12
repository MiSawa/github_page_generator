---

problem:
    round: SRM590
    level: [Div1 Med 500]
    rd: "15703"
    pm: "12079"
    name: XorCards
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12079&rd=15702
date: 2015/10/02
tags: [TopCoder, 行列, BitOperation, XOR]

---

### 概要

$n$ 個の(distinct とは限らない)非負整数たちが並んでいる.
このうちいくつかを(同じ数を区別して)選び, XOR を取ると, `limit` 以下だった.

こうなるような選び方は何通りあるか.


#### 制約

$ 1 \le n \le 50,\ 0 \le a_i \le 10^{15} $.

### 解法

数の比較は数字列の辞書順比較.
$X$ 未満の数というのは, $X$ と $i+1$ 桁目以降一致しているが, $X$ より $i$ 桁目が小さいというやつ.

二進でやると, $X$ のその桁が $1$ なのに, こっちは $0$ みたいな状況.

てぇことは, $i$ 桁目以降がそうなるように XOR を取ればよくて, それは連立方程式の解の個数を求めよってことで, つまり rank をがんばってねという.

せっかくだから二進行列ライブラリ作ろうと思ったけど, バグってつらかったので, 普通の行列ライブラリに投げた.


### ソースコード

~~~ cpp
// 176.71 pts

typedef int num;
const num EPS = 1;
typedef vector<num> vec;
typedef vector<vec> mat;
num rank(mat A){//{{{
    int n = size(A), m = size(A[0]);
    int r = 0;
    for(int i = 0; r < n && i < m; ++i){
        int piv = r;
        for(int j = r+1; j < n; ++j)
            if(abs(A[j][i]) > abs(A[piv][i])) piv = j;
        A[r].swap(A[piv]);
        if(abs(A[r][i]) < EPS) continue;
        for(int k = m-1; k >= i; --k)
            A[r][k] /= A[r][i];
        for(int j = r+1; j < n; ++j)
            for(int k = m-1; k >= i; --k)
                A[j][k] ^= A[r][k] * A[j][i];
        ++r;
    }
    return r;
}//}}}


long long XorCards::numberOfWays( vector<long long> number, long long limit ){
    ++limit;
    const int n = size(number);
    long long res = 0;
    rep(i, 60) if(limit>>i&1){
        long long x = limit ^ (1LL<<i);
        mat A(60, vec(n+1));
        rep(t, n) rep(j, 60) if(i <= j) A[j][t] = number[t]>>j&1;
        auto B = A;
        rep(j, 60) if(i <= j) A[j][n] = x>>j&1;
        int ra = ::rank(A);
        int rb = ::rank(B);
        int c = 0;
        rep(j, 60){
            bool ok = false;
            rep(i, n) if(A[j][i]) ok = true;
            c += ok;
        }
        if(ra == rb) res += 1LL<<(n-ra);
    }
    return res;
}
~~~

