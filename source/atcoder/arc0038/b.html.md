---

problem:
    contest: ARC038
    id: B
    name: マス目と駒
    url: http://arc038.contest.atcoder.jp/tasks/arc038_b
date: 2015/05/03
tags: [AtCoder, ゲーム, DP]

---

### 概要

二人で交互にターンが回ってくるゲームをする.

何マスかは障害物で埋まっている, `$H \times W$` のボードが与えられ,
最初は, 左上 `$(1, 1)$` に駒が置いてある.

毎ターン, 今ある場所から, 右, 右下, 下のいずれかのうち, 障害物で埋まっていない, `$H \times W$` のボード内である場所に駒を移動出来る.

移動できなくなった方が負け. どちらが勝つかを出力せよ.

#### 制約

`$1 \le H, W \le 100$`.


### 解法

"`DP[i][j] = 駒が (i, j) にある状態から, 先に移動するプレイヤーが勝てるか`"
で DP.

次のプレイヤーが勝てない状態に移動する事が出来るなら勝ち,
どう移動しても次のプレイヤーが勝てる状態になってしまうなら負け.

### ソースコード

~~~ cpp
bool solve(){
    int h, w; cin >> h >> w;
    vector<string> in(h+1, string(w+1, '#'));
    vector<vector<int>> win(h+1, vector<int>(w+1, false));
    rep(i, h) rep(j, w) cin >> in[i][j];
    for(int i = h-1; i >= 0; --i) for(int j = w-1; j >= 0; --j)
        rep(di, 2) rep(dj, 2) if(di+dj and in[i+di][j+dj] != '#')
            win[i][j] |= !win[i+di][j+dj];
    cout << (win[0][0] ? "First" : "Second") << endl;
    return true;
}
~~~

