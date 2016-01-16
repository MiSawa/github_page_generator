---

problem:
    round: SRM586
    level: [Div1 Med 500]
    rd: "15698"
    pm: "12692"
    name: History
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12692&rd=15698
date: 2015/10/07
tags: [TopCoder, 牛ゲー]

---

### 概要

いくつかの国がある.

国によって, "年号" は違うが, カレンダーは同じになっている.

また, 戦いの記録として, "国 A と B が, 王が x と y の時に戦った" のような記録が与えられる.
これは, 要するに, "国 A の年号 $ [x_s, x_t) $ と, 国 B の年号 $[y_s, y_t)$ の間に, 同じ年を表すものがある" という情報.

いくつかの, valid か知りたい戦いの記録があるので, 各クエリに対して, 正しいとわかっている記録と整合性が取れるか判定せよ.

#### 制約

$ 1 \le n \le 26 $, クソみたいな形式(マジで誰得なんだよ).

### 解法

仮想的な, 絶対的な年号を考えておく.

国 $ i $ に対して, $ i $ の年号で $ 0 $ 年が, 絶対的な年号でいつかを表す変数 $v_i$ を用意する.

すると, 各戦いの履歴は, $\exists a \in [x_s, x_t),\ \exists b \in [y_s, y_t) :\ v_i + a = v_j + b $ みたいな形になっている.

これはつまり, $ a + x_s \le b + y_t - 1 $ かつ $ b + y_s \le a + x_t - 1 $ のこと.

あとは, 牛ゲーをすればよい.


### ソースコード

~~~ cpp
// 305.79 pts

struct E{
    int s, t, c;
    E(int s, int t, int c) : s(s), t(t), c(c){}
};
vector<E> g;
void init(){ g.clear(); }
void add_constraints(int s, int c1, int t, int c2){
    int c = c2 - c1;
    g.emplace_back(t, s, c);
}
bool solve(){
    const int n = [&](){
        int k = 0;
        for(auto &e : g) chmax(k, max(e.s, e.t)+1);
        return k;
    }();
    vector<int> v(n, 0);
    for(int i = n+2, cont = true; cont and i >= 0; --i){
        cont = false;
        for(auto &e : g){
            if(v[e.t] > v[e.s] + e.c){
                v[e.t] = v[e.s] + e.c;
                cont = true;
            }
        }
        if(cont and !i) return false;
    }
    return true;
}

vector<pair<int, int>> read(const string battle){
    vector<pair<int, int>> res;
    stringstream ss(battle);
    char c; int t; ss >> c >> t;
    res.emplace_back(c - 'A', t);
    ss >> c >> c >> t;
    res.emplace_back(c - 'A', t);
    return res;
}

string History::verifyClaims( vector <string> dynasties, vector <string> battles_, vector <string> queries ){
    // as + [a[i], a[i+1]) = bs + [b[i], b[i+1])
    // as - bs = [b[i], b[i+1]) - [a[i], a[i+1])
    //        <= b[i] - a[i+1] + 1
    //        >= b[i+1] - a[i] - 1

    vector<string> battles;
    {
        string tmp = "";
        for(auto x : battles_) tmp += x;
        stringstream ss(tmp);
        for(string x; ss >> x; ) battles.emplace_back(x);
    }

    const int n = size(dynasties);
    vector<vector<int>> dyna(n);
    rep(i, n){
        stringstream ss(dynasties[i]);
        for(int x; ss >> x; ) dyna[i].emplace_back(x);
    }
    string res;
    for(auto q : queries){
        init();
        auto add_query = [&](const string battle){
            auto tmp = read(battle);
            rep(_, 2){
                add_constraints(
                        tmp[0].first, dyna[tmp[0].first][tmp[0].second],
                        tmp[1].first, dyna[tmp[1].first][tmp[1].second+1] - 1);
                swap(tmp[0], tmp[1]);
            }
        };
        add_query(q);
        for(auto b : battles) add_query(b);
        res += (solve() ? 'Y' : 'N');
    }

    return res;
}
~~~

