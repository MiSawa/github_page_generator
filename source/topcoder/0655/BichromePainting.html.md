---

problem:
    round: SRM655
    level: [Div1 Easy 250]
    rd: 16415
    pm: 13709
    name: BichromePainting
    url: http://community.topcoder.com/stat?c=problem_statement&pm=13709&rd=16415
date: 2015/03/10
tags: [TopCoder, 塗りつぶし, 逆から]

---

### 概要

$n \times n$ の, 各マスが白黒の状態を持つ盤面がある.
最初は全てのマスが白で, 終状態が指定されている.

$k \times k$ を白に塗りつぶす or 黒に塗りつぶす 事を何回かして, 指定された終状態に行けるか.

#### 制約

$1 \le k \le n \le 20$.

### 解法

塗りつぶしを後ろから逆算するやつ.

終状態は $k \times k$ の同色の部分がなければならない.
そこを最後に塗りつぶすと思うと, 終状態の一回前では, その $k \times k$ は何でもよかった事になる.

「ここはなんでも良かった」という状態が増える事で, 遷移できなくなる状態が増えることはないので, どこから塗っても OK.

### ソースコード

~~~ cpp
string BichromePainting::isThatPossible( vector <string> board, int k ){
    const int n = board.size();
    for(bool flg = true; flg; ){
        flg = false;
        rep(i, n-k+1) rep(j, n-k+1){
            set<char> use;
            rep(a, k) rep(b, k) use.emplace(board[i+a][j+b]);
            if(use.count('W') and use.count('B')) continue;
            if(use.size() == 1 and use.count('?')) continue;
            rep(a, k) rep(b, k) board[i+a][j+b] = '?';
            flg = true;
        }
    }
    string res = "Possible";
    rep(i, n) rep(j, n) if(board[i][j] != '?') res = "Impossible";
    return res;
}
~~~

