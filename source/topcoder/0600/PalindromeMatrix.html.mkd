---

problem:
    round: SRM600
    level: [Div1 Medium 600]
    rd: "15712"
    pm: "12875"
    name: PalindromeMatrix
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12875&rd=15712
date: 2015/09/29
tags: [TopCoder, Palindrome]

---

### 概要

$0, 1$ からなる二次元配列が与えられる.
但し, 縦横偶数サイズ.

いくつかのビットを反転して, 行のうち $r$ 個, 列のうち $c$ 個が回文になるようにしたい.

必要な反転の最小個数を求めよ.

#### 制約

$2 \le \mathit{A.size()}, \mathit{A[0].size()} \le 14$

### 解法
$\binom{14}{14/2}=3432$ なのはわかったが, $\binom{n}{n/2}^2 * n^2$ くらいのしか思いつかなかった.
どっちかだけ決めて DP とかも考えたけど, 上手くいかなかった.

解説を見た.
やっぱりうまくいくらしい.
行を決め打って, 列を $0, m-1, 1, m-2, \dots$ という順で決めればいい.
ここまで見て解き直し.

$\binom{n}{\mathit{rowCount}}$ を固定.
$(0, m-1)$ のそれぞれを使う/使わないの $4$ 通りに対して, そのためのコストがわかる.
あとは, DP で $\mathit{columnCount}$ 個以上になるようにがんばる.


### ソースコード

~~~ cpp
// 188.36
int PalindromeMatrix::minChange(vector<string> in, int rowCount, int columnCount ){//{{{
    const int n = size(in), m = size(in[0]);
    int res = n * m;

    rep(A, 1<<m) if(__builtin_popcount(A) == columnCount){
        vector<int> cost(n+2, n*m);
        cost[0] = 0;
        rep(i, n/2){
            const int ii = n - i - 1;
            vector<int> ncost(n+2, n*m);
            // in[i][*], in[ii][*] を使うか.
            rep(ti, 2) rep(tii, 2){
                int now = 0;
                rep(j, m/2){
                    const int jj = m - j - 1;
                    // in[i, ii][j, jj] をどうするか.
                    //  0--1   - i
                    //  |  |
                    //  3--2   - ii
                    //  j  jj
                    array<char, 4> cs = { in[i][j], in[i][jj], in[ii][jj], in[ii][j] };
                    UnionFind uf(4);
                    if(ti)      uf.unite(0, 1);
                    if(tii)     uf.unite(3, 2);
                    if(A>>j&1)  uf.unite(0, 3);
                    if(A>>jj&1) uf.unite(1, 2);
                    rep(a, 4) if(uf.find(a) == a){
                        int cnt = 0, ccnt = 0;
                        rep(b, 4) if(uf.find(b) == a) cnt += cs[a] == cs[b];
                        rep(b, 4) if(uf.find(b) == a) ccnt += cs[a] != cs[b];
                        now += min(cnt, ccnt);
                    }
                }
                rep(k, n) chmin(ncost[k + ti + tii], cost[k] + now);
            }
            swap(cost, ncost);
        }
        chmin(res, cost[rowCount]);
    }
    return res;
}//}}}
~~~

