---

problem:
    round: SRM594
    level: [Div1 Medium 550]
    rd: "15706"
    pm: "12809"
    name: FoxAndGo2
    url: http://community.topcoder.com/stat?c=problem_statement&pm=12809&rd=15706
date: 2015/10/01
tags: [TopCoder, 最大流]

---

### 概要

$ n \times n $ の盤面が与えられる.
各マスは, `o` か `x` か `.` のいずれかが書かれている.

`o` は白駒が置かれている, `x` は黒駒が置かれている, `.` は何もないことを表す.

現状では, `o` は隣り合っていない.

ここに, `x` をいくつか追加してもよい.
`四近傍に `.` が無い `o` は消滅する.

最終的に盤面に残っている駒の数の最大値はいくつか.


#### 制約

$ 3 \le n \le 50 $, `o` は少なくとも一つの `.` に隣接している.

### 解法

- `o` を取るには, いくつかの `.` を捨てなければならない.
- `o` を取ると $1$ 点獲得.
- `.` に `x` を置くと $1$ 点消費.

という状況.
こういうのは燃やす埋める系の最大流.

- `o` が S 側 <-> `o` を獲得する.
- `.` が S 側 <-> `x` を置く

と考えれば,

- S -> `o` に容量 $1$ (`o` を S 側に出来ないと $-1$ 点)
- `o` -> `.` に容量 $\infty$ (`o` を消すためには `.` が必要)
- `.` -> T に容量 $1$ (`.` を S 側にすると $-1$ 点)

で S -> T に流せばよい.

### ソースコード

~~~ cpp
// 443.09 pts

struct Dinic{//{{{
    typedef int Cap;
    static const Cap INF = 1<<29;

    struct E{//{{{
        int dst;
        Cap cap;
        int rev;
        E(int dst, Cap cap, int rev) : dst(dst), cap(cap), rev(rev) {}
    };//}}}
    vector<E> edges;
    vector<vector<E>> g;
    enum{ S = -1, T = -2 };
    int n;
    Dinic() : n(0){}

    inline void add_edge(const int &src, const int &dst, const Cap &cap){//{{{
        if(src == dst) return;
        edges.emplace_back(dst, cap, 0);
        edges.emplace_back(src,   0, 0);
    }//}}}
    inline void add_undirected_edge(int src, int dst, Cap cap){//{{{
        if(src == dst) return;
        edges.emplace_back(dst, cap, 0);
        edges.emplace_back(src, cap, 0);
    }//}}}

    inline int add_v(){ return n++; }
    inline vector<int> add_vs(int s){//{{{
        vector<int> res; res.reserve(s);
        for(int i = 0; i < s; ++i) res.emplace_back(add_v());
        return res;
    }//}}}

    void build(int &s, int &t){//{{{
        if(s < 0) s = add_v();
        if(t < 0) t = add_v();
        g.assign(n, vector<E>());
        for(int i = 0; i < edges.size(); i += 2){
            E &e = edges[i], &re = edges[i^1];
            int &u = re.dst, &v = e.dst;
            if(u < 0) u = u == -1 ? s : t;
            if(v < 0) v = v == -1 ? s : t;
            e.rev = g[v].size(); re.rev = g[u].size();
            g[u].emplace_back(e); g[v].emplace_back(re);
        }
    }//}}}

    vector<int> level, iter;
    Cap dfs(const int &s, const int &u, Cap flow){//{{{
        if(s == u or flow == 0) return flow;
        Cap sum = 0;
        for(int &i = iter[u]; i >= 0; --i){
            E &e = g[u][i], &re = g[e.dst][e.rev];
            const int &v = e.dst;
            if(level[v] >= level[u] or re.cap <= 0) continue;
            Cap f = dfs(s, v, min(flow - sum, re.cap));
            if(f <= 0) continue;
            re.cap -= f; e.cap += f;
            sum += f;
            if(sum == flow) break;
        }
        return sum;
    }//}}}
    Cap run(int s = -1, int t = -2){//{{{
        build(s, t);
        vector<int> q(n);
        for(Cap flow = 0; ; ){
            level.assign(n, -1);
            int ql = 0, qr = 0;
            level[q[qr++] = s] = 0;
            while(ql != qr && level[t] == -1){
                int u = q[ql++];
                for(auto &e : g[u]) if(e.cap > 0 && level[e.dst] == -1)
                    level[ q[qr++] = e.dst ] = level[u] + 1;
            }
            if(level[t] == -1) return flow;
            iter.resize(n);
            for(int u = 0; u < n; ++u) iter[u] = (int)(g[u].size()) - 1;
            flow += dfs(s, t, INF);
        }
    }//}}}
};//}}}

int FoxAndGo3::maxEmptyCells( vector <string> board ){
    Dinic mf;
    const int h = size(board), w = size(board[0]);
    vector<vector<int>> vs(h);
    rep(i, h) vs[i] = mf.add_vs(w);
    const vector<int> dxy = {+1, 0, -1, 0, +1};
    rep(i, h) rep(j, w) if(board[i][j] == 'o'){
        rep(dir, 4){
            int ii = i + dxy[dir], jj = j + dxy[dir+1];
            if(0 <= ii and ii < h and 0 <= jj and jj < w and board[ii][jj] == '.')
                mf.add_edge(vs[i][j], vs[ii][jj], 1<<15);
        }
    }
    rep(i, h) rep(j, w) if(board[i][j] == 'o') mf.add_edge(mf.S, vs[i][j], 1);
    rep(i, h) rep(j, w) if(board[i][j] == '.') mf.add_edge(vs[i][j], mf.T, 1);
    int res = -mf.run();
    rep(i, h) rep(j, w) if(board[i][j] != 'x') ++res;
    return res;
}
~~~

