---

problem:
    round: SRM584
    level: [Div1 Med 600]
    rd: "15696"
    pm: "12641"
    name: Excavations
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12641&rd=15696
date: 2015/10/21
tags: [TopCoder, DP]

---

### 概要

いくつかの構造物が埋まっている.

$ i $ 番目の構造物は, $ \mathit{kind}_i $ という種類のもので, 位置 $ i $ の深さ $ \mathit{depth}_i $ に埋まっている.

いくつかの場所を, 最大 $ D $ 掘れる機械で掘る. (この $ D $ は与えられない)

すると, 見つかったのは $ K $ 個の構造物で, 種類の集合(i.e. 重複を排除したもの)は $ \mathit{found} $ であった.

掘った場所の組合せとしてありうるものの個数を答えよ.

#### 制約

$ 1 \le n \le 50,\ 1 \le \mathit{depth}_i \le 10^5 $.

### 解法

とりあえず, 掘る深さ $ D $ を固定し, その深さの構造物を少なくとも一つ掘り出すとする.

"found に含まれるやつのうち, 深さ $ D $ 以下のやつらを, 全ての種類が出るように, かつその深さのを一つ以上含むように $ i $ 個掘り出すやり方" は, DP をすれば求まる.

"その深さ $ D $ より大のやつらを, $ i $ 個掘り出すが, そのうち一番浅い所に, found に含まれないものがあるやり方" も, DP をすれば求まる.

あとは掛けて足せばよい.

"そのうち一番浅い所に, found に含まれないものがある" を忘れて 1WA.

"今見てる kind, いまいくつ掘ったか, max 掘り出した深さ, min 掘り出せなかった深さ" を持ってメモ化再帰するのが楽っぽい.

### ソースコード

~~~ cpp
// 187.12 pts

long long Excavations::count( vector <int> kind, vector <int> depth, vector <int> found_, int K ){
    vector<vector<long long>> C(200, vector<long long>(200));
    rep(i, 200){
        C[i][0] = C[i][i] = 1;
        for(int j = 1; j < i; ++j) C[i][j] = C[i-1][j-1] + C[i-1][j];
    }

    const int n = size(kind);
    const int t = 55;
    vector<int> found(t);
    for(auto f : found_) found[f] = true;

    vector<int> ds;
    rep(i, n) if(found[kind[i]]) ds.emplace_back(depth[i]);
    sort(all(ds));
    ds.erase(unique(all(ds)), end(ds));

    long long res = 0;

    for(auto d : ds){
        // この深さ掘る. この深さのを少なくとも一つ使う.
        unordered_map<int, int> use, use_d;
        vector<pair<int, int>> not_use;
        rep(i, n){
            if(found[kind[i]]) if(depth[i] <= d) ++use[kind[i]];
            if(found[kind[i]]) if(depth[i] == d) ++use_d[kind[i]];
            if(depth[i] > d) not_use.emplace_back(depth[i], found[kind[i]]);
        }
        sort(all(not_use));
        if(use.size() != found_.size()) continue;
        // それぞれの kind を少なくとも一つ使い, 少なくとも一つ d のを使う
        vector<long long> res_1;
        {//{{{
            array<vector<long long>, 2> dp;
            dp[0] = dp[1] = vector<long long>(K+1, 0);
            dp[0][0] = 1;
            for(auto kv : use){//{{{
                const int k = get<0>(kv);
                array<vector<long long>, 2> qb;
                qb[0] = qb[1] = vector<long long>(K+1, 0);
                const int A = use_d[k], B = use[k] - use_d[k];
                rep(i, A+1) rep(j, B+1) if(i+j > 0){
                    rep(now, K+1) if(now+i+j <= K){
                        if(i == 0) qb[0][now+i+j] += dp[0][now] * C[A][i] * C[B][j];
                        else       qb[1][now+i+j] += dp[0][now] * C[A][i] * C[B][j];
                        qb[1][now+i+j] += dp[1][now] * C[A][i] * C[B][j];
                    }
                }
                swap(dp, qb);
            }//}}}
            res_1 = dp[1];
        }//}}}
        // d よりでかいのを, "一番最初は found でないの" という条件付きで使う.
        vector<long long> res_2;
        {//{{{
            tr << not_use << endl;
            array<vector<long long>, 2> dp;
            dp[0] = dp[1] = vector<long long>(K+1, 0);
            dp[0][0] = 1;
            for(auto df : not_use){
                array<vector<long long>, 2> qb = dp;
                rep(now, K){
                    if(get<1>(df)){
                        qb[1][now+1] += dp[1][now];
                    }else{
                        qb[1][now+1] += dp[0][now];
                        qb[1][now+1] += dp[1][now];
                    }
                }
                swap(dp, qb);
            }
            res_2 = dp[1];
            res_2[0] = 1; // 何も使わないのは O.K.
        }//}}}
        rep(k, K+1) res += res_1[k] * res_2[K-k];
    }

    return res;
}
~~~

