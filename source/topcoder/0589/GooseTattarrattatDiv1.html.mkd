---

problem:
    round: SRM589
    level: [Div1 Easy 250]
    rd: "15701"
    pm: "12730"
    name: GooseTattarrattatDiv1
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12730&rd=15701
date: 2015/10/06
tags: [TopCoder, 回文]

---

### 概要

小文字からなる文字列が与えられる.
回文にしたい.

アルファベット一文字を選び, 別の一文字に置換する事ができる.
その置換にかかるコストは, 置換された文字の数.

コストを最小化せよ.

#### 制約

$ 1 \le n \le 50 $.

### 解法

`s[i]` と `s[n-i-1]` を一致させなきゃいけないので, とりあえず union-find する.

各連結成分について, "最も s 内で多く使われている文字" に, その他の文字を置換するのがよい.

ところで, 主人公の名前, 難しくないですか???
"T から始まる長い文字列は主人公の名前" と思って読み飛ばしてしまっていたけれど, よく見たら回文だった.


### ソースコード

~~~ cpp
// 241.95 pts

int GooseTattarrattatDiv1::getmin( string s ){
    UnionFind uf(256);
    const int n = size(s);
    rep(i, n) uf.unite(s[i], s[n-i-1]);
    vector<int> mx(256);
    rep(i, n) ++mx[s[i]];
    rep(i, n) chmax(mx[uf.find(s[i])], mx[s[i]]);
    int res = n;
    rep(i, 256) if(uf.find(i) == i) res -= mx[i];
    return res;
}
~~~

