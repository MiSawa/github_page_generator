---

problem:
    round: SRM588
    level: [Div1 Med 450]
    rd: "15701"
    pm: "12714"
    name: KeyDungeonDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12714&rd=15700
date: 2015/10/07
tags: [TopCoder, bitDP]

---

### 概要

鍵穴がいくつかついた扉がいくつかある.
それぞれの扉の先には, いくつかの鍵が置いてある.

鍵穴は赤と緑のニ種類がある.
一方, 鍵は赤と緑と白の三種類がある.

赤や緑の鍵は, それぞれその色の鍵穴にのみ使える.
白は両方に使える.

扉を開けると, 使った鍵はこわれてしまう.

最初に所持している鍵と, 扉の情報が与えられるので,
所持している, 壊れていない鍵の合計数を最大化せよ.


#### 制約

$ 1 \le \mathit{doors} \le 12,\ 1 \le \mathit{doorR}, \mathit{doorG}, \mathrm{e.t.c} \le 10 $.

### 解法

BitDP する.

開けた扉を知っていると, 今持っている鍵の個数がわかる.
あとは, 赤緑白のうち二つの情報がわかればよい.

白い鍵はなるべく多くしたいので, `dp[開けた扉][赤] = max 白` のようにすればよい.


### ソースコード

~~~ cpp
// 358.87 pts

int KeyDungeonDiv1::maxKeys( vector <int> doorR, vector <int> doorG, vector <int> roomR, vector <int> roomG, vector <int> roomW, vector <int> keys ){
    const int n = size(doorR);
    vector<vector<int>> dp = vector<vector<int>>(1<<n, vector<int>(140, -(1<<30)));
    dp[0][keys[0]] = keys[2];

    int res = 0;

    rep(A, 1<<n) rep(r, 140) if(dp[A][r] >= 0){
        int sum = keys[0] + keys[1] + keys[2];
        rep(i, n) if(A>>i&1){
            sum += roomR[i] - doorR[i];
            sum += roomG[i] - doorG[i];
            sum += roomW[i];
        }
        chmax(res, sum);
        int nowR = r, nowW = dp[A][r], nowG = sum - nowR - nowW;

        rep(i, n) if(!(A>>i&1)){
            int useR = min(nowR, doorR[i]);
            int useG = min(nowG, doorG[i]);
            int useW = (doorR[i] - useR) + (doorG[i] - useG);
            if(useW > nowW) continue;
            int nexR = nowR - useR + roomR[i];
            int nexG = nowG - useG + roomG[i];
            int nexW = nowW - useW + roomW[i];
            chmax(dp[A|(1<<i)][nexR], nexW);
        }
    }
    return res;
}
~~~

