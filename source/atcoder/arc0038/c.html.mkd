---

problem:
    contest: ARC038
    id: C
    name: 茶碗と豆
    url: http://arc038.contest.atcoder.jp/tasks/arc038_c
date: 2015/05/03
tags: [AtCoder, ゲーム, segtree, grundy数]

---

### 概要

二人で交互にターンが回ってくるゲームをする.

`$0, \dots, n-1$` の `$n$` マスが横並びに並んでおり,
各マス `$i$` には駒が `$A_i$` 個置いてある.

毎ターン, 駒を一つ選択し, 動かす.
但し, `$i$` 番目のマスから動かせる先は, `$i-C_i, i-C_i+1, \dots, i-1$` のいずれか.

動かせなくなったプレイヤーの負け.
先手と後手どちらが勝ちか.

#### 制約

`$1 \le n \le 10^5$`, `$1 \le C_i \le i$`, `$0 \le A_i \le 10^9$`, `$1 \le \sum A_i$`.

### 解法

駒は独立なので, 独立なゲームを `$\sum A_i$` 個同時進行していると思おう.

駒が一つの時, grundy 数を求めればよい.
同時進行しているので, 全ての xor を取れば答え.

但し, grundy 数を求めるのに, `$O(n^2 \log n)$` とかかかると死ぬので, ここを高速化する必要がある.

`t[x] = grundy数が x な中で最も右の index`
と置き, これを更新していく事を考える.
`$i-1$` まで処理した時, `$i$` の grundy 数 `$y$` は,

`$$\min_{x < y} t[x] \ge i - C_i$$`

を満たす最小の `$y$`. これを二分探索すればよい.


### ソースコード

~~~ cpp
template<typename T>
struct SegTree{//{{{
    const T zero;
    vector<T> tree;
    int offset, N;

    T propagate(const T &l, const T &r){ return min(l, r); }

    SegTree(int n, const T zero = T()) : zero(zero){
        N = 1;
        while(N < n) N <<= 1;
        tree.assign(N*2, zero);
        offset = N-1;
    }
    T at(int i){ return tree[i + offset]; }
    void set(int i, const T &x){
        i += offset;
        tree[i] = x;
        while(i){
            i = (i-1) >> 1;
            tree[i] = propagate(tree[i*2+1], tree[i*2+2]);
        }
    }
    T sum(const int &l, const int &r){ return sum(l, r, 0, 0, N); }
    T sum(const int &l, const int &r, const int &k, const int &ll, const int &rr){
        if(r <= ll || rr <= l) return zero;
        if(l <= ll && rr <= r) return tree[k];
        const int mm = (ll + rr) >> 1;
        if(r <= mm) return sum(l, r, k*2+1, ll, mm); ////
        if(l >= mm) return sum(l, r, k*2+2, mm, rr); ////
        return propagate(sum(l, r, k*2+1, ll, mm), sum(l, r, k*2+2, mm, rr));
    }
};//}}}

bool solve(){
    int n; cin >> n;
    vector<int> c(n+1), a(n+1);
    rep(i, n) cin >> c[i+1] >> a[i+1];
    SegTree<int> seg(n+10, -1);
    vector<int> grundy(n+1);
    int res = 0;
    rep(i, n+1){
        grundy[i] = 0;
        for(int d = 1<<20; d; d >>= 1)
            if(grundy[i]+d < n+10 and seg.sum(0, grundy[i] + d) >= i - c[i])
                grundy[i] += d;
        seg.set(grundy[i], i);
        if(a[i]%2) res ^= grundy[i];
    }
    cout << (res == 0 ? "Second" : "First") << endl;
    return true;
}
~~~

