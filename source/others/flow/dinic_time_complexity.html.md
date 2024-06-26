---

title: Dinic 法とその時間計算量
date: 2021/01/15
tags: [フロー, 最大流]
secret: false
description: Dinic 法の実装手法の解説と時間計算量の解析、特殊なグラフ上での時間計算量の解析。

---
<div style="display:none">$$
\gdef\vec#1{\mathbf{#1}}
\gdef\vecb{\vec{b}}
\gdef\veczero{\vec{0}}
\gdef\vecf{\vec{f}}
\gdef\exG{\overleftrightarrow{G}}
\gdef\label{\operatorname{label}}
\gdef\ce{\operatorname{current\_edge}}
\gdef\fe{\operatorname{first\_edge}}
\gdef\ne{\operatorname{next\_edge}}
$$</div>

# Dinic 法とその時間計算量
## 要約

- 一般に, `$n$` 頂点 `$m$` 辺のグラフと実数の辺容量 `$u$` が与えられたとき, このネットワーク上での Dinic 法の計算量は `$O(n^2 m)$` である.
- 辺容量が整数のとき,
  - 最大流を `$F$` とすると, `$O(F m)$` でもある.
  - 辺容量の平均値が `$k$` のとき, `$O(k m^{3/2})$` でもある.
  - 辺容量の最大値が `$k$` で多重辺が無いとき, `$O(k n^{2/3}m)$` でもある.
  - 各頂点を通れるフロー量の平均値が `$k$` 以下, すなわち `$k \ge \operatorname{avg}_v \min(\sum_{e \in \delta^+(v)} u_e, \sum_{e \in \delta^-(v)} u_e)$` なとき, 計算量は `$O(k \sqrt{n} m)$` でもある.[^delta]
  - 二部マッチングのときは, 上が `$k = 1$` で成立し, `$O(\sqrt{n} m)$`.
  - 容量が整数は必須だけれど, 他の条件は高々定数個の例外があっても OK.
- 無向基礎グラフにおける `$s$`-`$t$` 最**長**路長を `$l$` とすると, `$O(l^2 m)$` である.
  - 二部グラフにおいて, 頂点分割の小さい方の頂点数を `$s$` とすれば, `$O(s^2 m)$` である.
- 動的木を使うと, 一般のグラフで `$O(n m \log n)$` になる.
- 実装をミスると指数オーダー.

この記事では, 理論的な側面やその背景にはあまり立ち入らない.
定数倍なども特に気にせず, "教科書に書いてある Dinic 法" と "よくある Dinic 法の実装" の間を埋めることと, その計算量を解析することを目的としている.

昔書いた記事 [最大流問題について](https://topcoder-g-hatena-ne-jp.jag-icpc.org/Mi_Sawa/20140311/), [最大流問題について その3](https://topcoder-g-hatena-ne-jp.jag-icpc.org/Mi_Sawa/20140320.html) の一部を詳しく説明した感じです.

[^delta]: `$\delta^+(v)$`: `$v$` から出る辺全体の集合, `$\delta^-(v)$`: `$v$` へ入る辺全体の集合.

## 記号
- `$\exG$`: `$G$` に逆辺を追加したやつ.
- `$G_\vecf$`: `$\vecf$` に関する残余ネットワーク. `$\exG$` に容量を, 順辺は `$u_e - f_e$`, 逆辺は `$f_e$` で入れたやつ.
- `$m$`: `$\exG$` の辺の数. つまり元のグラフの倍.

`$O(n + m) = O(m)$` と簡略にしたいので(必要ならば連結成分を取り出して), `$n = O(m)$` を仮定する.

## Dinic 法の概要

Dinic 法は, 暫定解であるフロー `$\vecf$` を持ち, 次の2つを `$s$` から `$t$` への残余パスが無くなるまで繰り返すアルゴリズムである.

**dual step**:[^whydual]

残余ネットワーク `$G_\vecf$` 上で, `$s$` から `$t$` への最短経路 DAG, すなわち最短 `$s$`-`$t$` パスに含まれうる頂点/辺のみからなるグラフを求める.
ここで, 各辺のコストは `1`, つまり通る辺の個数を最小化する経路を求めるものとする.

[^whydual]: これが dual step と呼ばれているのを見たことはない. しかし, 線形計画問題として定式化したときの双対の `$\varepsilon$` 緩和を考え, 主の解を更新せずに双対の解を更新することで, なるべく小さい `$\varepsilon$` に対する `$\varepsilon$`-optimality を満たすようにする操作である.

**primal step**[^whyprimal],[^blocking]

`$H$` を前回の dual step で求めた DAG で初期化し, `$H$` が空グラフになるまで, 次を繰り返す.

1. H 上での `$s$`-`$t$` パスを一つ求める.
2. そのパス上に流せるだけ流す.
3. 残余容量が `$0$` になった辺と, `$s$` から辿り着けない頂点, `$t$` へ辿り着けない頂点, それらに隣接する辺を `$H$` から取り除く.

この繰り返しの後, dual step で求めた DAG 上で流したフローを, 暫定解である `$G$` 上のフロー `$\vecf$` に反映する. これにより, 残余ネットワーク, 特に逆辺の残余容量も更新される.

[^whyprimal]: dual step に対応し, こちらは `$\varepsilon$`-optimality を保ちつつ主問題の目的関数値を増大させている.
[^blocking]: これは dual step で求めた DAG の上でブロッキングフローを求める操作である.


## 計算量

さて, このアルゴリズムの計算量を調べよう.

いま, ひとつの dual-primal step に注目する. この dual step での `$s$` から `$v$` への最短路長(路が無いならば `$\infty$`)を `$\label(v)$`, `$l = \label(t)$` とすると, `$s$` から到達可能な任意の辺 `$(u, v)$` に対し, `$\label(u) + 1 \ge \label(v)$` である. `$s$`-`$t$` 最短路 DAG を `$H_0$` とする.
このとき, `$s$` から辿り着け, `$t$` へ辿り着けるような辺 `$(u, v)$` が `$H_0$` に含まれるためには, `$\label(u) + 1 = \label(v)$` であることが必要である.
直後の primal step の後, `$H$` 上のフローを `$\vecf$` に反映する際に, `$G_\vecf$` の辺が追加/削除されるが,

- `$G_\vecf$` に追加されうる辺 `$(u, v)$` は, `$H_0$` の逆辺, 従って `$\label(u) = \label(v)+1$` となる辺のみである.
- `$H_0$` 上の任意の `$s$`-`$t$` パスは, その少なくとも一辺が`$G_f$` から削除される.

の2つが成り立つ.

更新後の `$G_\vecf$` の `$s$`-`$t$` パスを一つとる. このパス上での `$\label$` の値からなる列を考えると, 高々 `$1$` ずつしか増加しない初項 `$0$`, 末項 `$l$` の数列であるから, これが長さ `$l$` 以下となるには, このパスに含まれる全ての辺 `$(u, v)$` で `$\label(u) + 1 = \label(v)$` を満たし, 長さ `$l$` となるしかない. しかし, このような辺は新たに `$G_\vecf$` に加わったものではないので, このパス更新前の `$G_\vecf$` の長さ `$l$` の `$s$`-`$t$` パス, 従って `$H_0$` 上の `$s$`-`$t$` パスであったことになり, primal step でその少なくとも一辺が削除されたことに矛盾する. 従って, 更新後の `$G_\vecf$` の任意の `$s$`-`$t$` パスの長さが `$l$` より長く, 次の dual step では `$s$` から `$t$` への最短路長が真に長くなる事がわかった.
`$s$`-`$t$` パスの長さは高々 `$n-1$` であるから, dual-primal step は高々 `$n-1$` 回しか繰り返されないことがわかる.

次に, dual step は, 幅優先探索で `$O(m)$` で実現できる.

最後に, primal step の計算量を考えよう.
`$H$` は最短経路 DAG であったから, `$s$` から順に出る辺を辿るだけで, `$O(n)$` で `$s$`-`$t$` パスを一つ求めることが出来る. `$H$` の更新についても, 残余容量が `$0$` になった辺を削除した後, 入次数や出次数が `$0$` になった頂点とそれに接続する辺を削除することを繰り返せばよいから, 取り除く頂点と辺の数の線形時間, 従ってこの primal step 内で合計 `$O(m+n)$` で行える.
フローを流す度に辺が少なくとも一本削除されるから, 合計で `$O(nm + m + n) = O(nm)$` となる.

dual-primal step は高々 `$n-1$` 回行われ, dual step が `$O(m)$`, primal step が `$O(nm)$` であったから, 合計で `$O(n^2 m)$` である.


## Current-Edge data structure を用いた実装方法

上では, `$H$` を毎 dual-primal step で陽に作り, そこから頂点や辺を削除していく方針を見た.
しかし, `$H$` や `$H$` 上のフローを陽に保持することなく, `$\exG$` と暫定解としての `$\exG$` 上のフロー, いくつかの補助変数を用いて実装するのが _非常に_ 一般的である.

**dual step**:

dual step では陽に DAG を作らず, `$s$` からの距離(もしくは `$t$` への距離. この場合符号を反転するか大小関係を逆にして読むこと.) `$\label(u)$` を求める.
すると, `$s$`-`$t$` パスは, その全ての辺で `$\label(u) + 1 = \label(v)$` を満たすとき, またその時に限り, 所望の DAG に含まれる.

**primal step**:
前提として, 隣接リストなどのデータ構造を用い, 各頂点 `$u$` に対し, `$\exG$` 上の `$u$` から出る辺 `$(u, v)$` を適当に固定された順序でイテレート出来るものとする.
すなわち, "最初の辺" を返す `$\fe(v)$` と, "あるならば次の辺, 無いなら特別な値 `$\bot$` を返す" `$\ne(e)$` が, それぞれ `$O(1)$` で出来るものとする.

残余容量が正で, `$\label(u) + 1 = \label(v)$` である辺 `$e = (u, v)$` を, admissible であると呼ぶ.
Admissible であり, `$s$` から admissible な辺のみを用いて辿りつける始点と `$t$` に admissible な辺のみを用いて辿り着ける終点を持つ辺が, 概要の節で述べた `$H$` の辺に対応する.

primal step では, dual step で求めた `$\label(\cdot)$` に加えて `$\ce(\cdot)$` を用る.
また, 概要の節でのアルゴリズムの "3. 残余容量が `$0$` になった辺と, `$s$` から辿り着けない頂点, `$t$` へ辿り着けない頂点を `$H$` から取り除く." は, 遅延して実行する.

最初に, `$v \neq t$` に対しては `$\ce(v) = \fe(v)$`, `$v = t$` に対しては `$\ce(t) = \bot$` と初期化する.
この `$\ce$` を用いて, 概要の節の `$H$` 上のパスを一つ求めるアルゴリズムは, 次のように実現できる.

1. `$i = 0, v_0 = s$` とする.[^fromt]
2. `$\ce(v_i) \neq \bot$` だが `$\ce(v_i)$` が admissible でないとき:
   1. `$\ce(v_i)$` を `$\ne(\ce(v_i))$` で更新する.
   2. 2.へ戻る.
3. `$\ce(v_i) \neq \bot$` のとき:
   1. `$\ce(v_i) = (v_i, v_{i+1})$` となるよう `$v_{i+1}$` を定める
   2. `$i$` を `$i+1$` に更新する
   3. 2.へ戻る.
4. `$v_i = s$`, すなわち `$i = 0$` のとき:
   1. admissible な辺からなる `$s$`-`$t$` パスを発見出来なかったことを報告し, 終了する.
5. `$v_i \neq t$` のとき:
   1. `$i$` を `$i-1$` で更新する.
   2. `$\ce(v_i)$` を `$\ne(\ce(v_i))$` で更新する.
   3. 2.へ戻る.
6. `$v_i = t$` のとき:
   1. パス `$s = v_0, \dots, v_i = t$` を報告し, 終了する.

[^fromt]: 今回は `$s$` から出発するようにしたが, `$t$` から始めてもよい. その場合さまざまなものが逆転する.

このアルゴリズムにおける重要な不変条件として,

- 辺 `$(u, v)$` が admissible かつ `$v$` から `$t$` への admissible な辺から成るパスがあるならば, それは `$\ce(v)$` 以降の辺である. (逆は必ずしも真ではない)

がある. この不変条件が正しいことは, `$\ce(v_i)$` を更新する箇所, 具体的には 2.1. と 5.2. で保たれることと, ある時点で admisslbe で無い辺は以後 admissible にならないことを確認すればわかる.
また, このアルゴリズム自体が正しいことは, 不変条件から示せる.

- 3.2. で `$i$` に加えた `$1$` は, 6.1. で報告するパスの長さか, 5.1. で減らされる `$1$` に寄与する.
- 2.1, 5.1. で `$\ce(\cdot)$` を変更している.
- 1., 4., 6. はそれぞれ高々1回しか実行されない.

から, このアルゴリズムの計算量は, 最終的に報告するパスの長さと `$\ce(\cdot)$` を変更した回数の和に関する線形時間である.

さて, このアルゴリズムで報告されたパスに対して, 含まれる辺の残余容量の最小値と同じだけフローを `$\vecf$` に追加し, これによって残余容量が `$0$` となった辺 `$e=(u, v)$` に対し, `$\ce(u)$` を `$\ne(e)$` で更新する.[^updateisunnecessary]
すると, 次のパスを探索する際は, 上のアルゴリズムを `$\ce(\cdot)$` を再度初期化することなく適用することができる.
このことの正しさは, やはり不変条件が保たれることからわかる.

Admissible な辺からなる `$s$`-`$t$` パスが無くなるまでこれを実行すると, `$\ce(\cdot)$` は合計高々 `$m$` 回変更される[^whyOm]が, 6.1. によってパスを得る度に一度変更するから, このアルゴリズムが実行されるのは高々 `$m+1$` 回である. また, 各実行で `$O(n)$` に加えて `$\ce(\cdot)$` の変更回数に関する線形時間かかるから, 合計 `$O(m n + m) = O(nm)$` かかる.

dual-primal step が高々 `$n-1$` 回なのは前の解析と変わらないから, 合計 `$O(n^2 m)$` である.

[^updateisunnecessary]: `$\ce(u)$` は更新しなくても, 計算量は変わらない. ここでは, 計算量解析の簡単さを優先した.
[^whyOm]: `$\ce(\cdot)$` は辺のイテレータで, 各辺一度しか出てこない.



## Current-Edge data structure の実装を間違えた場合

次の C++ コードは, 上のアルゴリズムの非常によくある実装の一部を抜き出したものである.

```cpp
Flow primal(const Flow current_path_cap, const size_t v) {
  if (v == target) return current_path_cap;
  for (size_t &i = current_edge[v]; i < edges[v].size(); ++i) {
    auto &e = edges[v][i];
    if (e.flow < e.capacity && label[e.to] == label[e.from] + 1) {
      // recurse with e
      const Flow f = primal(std::min(e.capacity - e.flow, current_path_cap), e.to);
      if (f == 0) continue;
      e.flow += f;
      e.reverse->flow -= f;
      return f;
    }
  }
  return 0;
}
```

この `size_t & i = current_edge[v]` の `&` は **非常に** 重要である.
これを忘れた場合, 前節で説明した Current-Edge data structure を使わず, 全ての残余パスを調べることになる.
`$s$`-`$t$` パスの本数は `$n$` の指数オーダーありうるので, 指数オーダーのアルゴリズムになってしまう.

このような間違った実装法で `$\Theta(2^{n/2})$` になるようなインスタンスを生成するプログラム, `$n = 100$` で生成されたインスタンスと, 実際に間違った実装法での実装が [ここ](https://gist.github.com/MiSawa/47b1d99c372daffb6891662db1a2b686) に置いてある.
primal step, dual step はそれぞれ `$s$` 側と `$t$` 側から出来るため, `$4$` 通りの実装方法があるが, そのいずれでも `$\Theta(2^{n/2})$` となっているハズなので, 自分の実装が不安な場合はこの生成器を使って試してみるとよいだろう.

## 動的木を使った実装

この節は読み飛ばしても差支えない.

前節の `$\ce(\cdot)$` は, `$s$` を葉の一つ, `$\ce(v) = \bot$` な頂点を根とする内向き森になっている.
この森を GetRoot, GetLastEdgeInPath[^GetLastEdgeInPath], Link, Cut, GetValue[^GetValue], AddToPath[^AddToPath], MinNodeOfPath[^MinNodeOfPath] を `$O(\log n)$` でサポートする動的木, 例えば Sleator-Tarjan Link/Cut tree で管理すると, `$O(\log n)$` について一度 `$\ce(\cdot)$` を更新する(Link/Cut)か admissible なパスにフローを流す(MinNodeOfPath + AddToPath)ことができる.

[^GetLastEdgeInPath]: `$v$` に対し `$v$` から `$v$` を含む木の根へのパスに含まれる辺で, 最も根に近いもの.
[^GetValue]: 各頂点には値(具体的には `$\ce(v)$` の残余容量)が付与されている. `$v$` に対し, この付与された値を返す.
[^AddToPath]: `$v$` に対し `$v$` から `$v$` を含む木の根へのパスに含まれる頂点に付与された値に `$x$` を足し込む.
[^MinNodeOfPath]: `$v$` に対し `$v$` から `$v$` を含む木の根へのパスに含まれる頂点のうち, 付与された値が最も小さいものを返す. 複数あるならば, 最も根に近いものを返す.

具体的には, 動的木を辺数 `$0$` で初期化し, 各頂点には開始時点の残余容量で値を付与した後, 次を行う.

1. `$v = \operatorname{GetRoot}(s)$` とする.
2. `$\ce(v) \neq \bot$` だが `$\ce(v)$` が admissible でないとき:
   1. `$\ce(v)$` を `$\ne(\ce(v))$` で更新する.
   2. 2.へ戻る.
3. `$\ce(v) \neq \bot$` のとき:
   1. `$\operatorname{Link}(\ce(v))$` する
   2. 1.へ戻る.
4. `$v = s$` のとき:
   1. admissible な辺からなる `$s$`-`$t$` パスを発見出来なかったことを報告し, 終了する.
5. `$v \neq t$` のとき:
   1. `$e = (u, v) = \operatorname{GetLastEdgeInPath}(s)$` とする.
   2. `$\operatorname{Cut}(e)$` する.
   3. `$\ce(u)$` を `$\ne(e)$` で更新する.
   4. 1.へ戻る.
6. `$v = t$` のとき:
   1. `$u = \operatorname{MinNodeOfPath}(s)$` とし, `$f = \operatorname{GetValue}(u)$` とする.
   2. `$\operatorname{AddToPath}(s, -f)$` する.
   3. `$u = \operatorname{MinNodeOfPath}(s)$` とし, `$e = \ce(u)$`, `$f = \operatorname{GetValue}(u)$` とする.
   4. `$f \neq 0$` ならば 1.へ戻る.
   5. `$\operatorname{Cut}(e)$` する.
   6. `$\ce(u)$` を `$\ne(e)$` で更新する.
   7. 6.3.へ戻る.

このアルゴリズムの終了時点での各頂点に付与された値に応じて, 暫定解のフロー量を更新する.
このアルゴリズムは primal step を `$O(m \log n)$` で実装し, 全体として `$O(n m \log n)$` の最大流アルゴリズムを与えている.


## 特殊なネットワーク上での計算量
**!!!以下, 辺容量の整数性を仮定する!!!**

動的木を用いた実装については一旦忘れて, 特殊なグラフ上で Current-Edge data structure を用いた実装を実行したときに, Dinic 法の計算量がより小さくなることを見る.

### 最大流量に関する計算量

辺容量が整数であるから, フローを更新する際, その値は少なくとも `$1$` 増える.
従って, 最大流量が `$F$` であるとすると, フローを更新できるのは高々 `$F$` 回である.
dual-primal step 一回につき, 少なくとも一回フローを更新できるから, dual step は合計で `$O(F m)$`.

一方, primal step はパスを発見する度にフローを更新する. この計算量は, 発見するパスの長さと `$\ce(\cdot)$` の更新回数の和に関する線形時間であった.
ここで, パスの長さは `$O(n)$`, パスを発見する回数は全 dual-primal step 合わせて `$O(F)$` であり, `$\ce(\cdot)$` の更新回数は一つの primal step につき `$O(m)$` であったから,
全て合わせて `$O(F m + F n + F m) = O(F m)$` である.


### 辺容量が高々定数/辺容量の平均値が高々定数

最大流量に関する計算量の議論は, Dinic 法の実行中でも成立する. すなわち, 実行中のある時点で, 残余グラフ上の最大流量が `$F (\ge 1)$` となったとき, その後 `$O(F m)$` でアルゴリズムは終了する.
最大流-最小カット定理から, 実行中のある時点でのある残余カットの容量が `$F (\ge 1)$` であるとき, その後 `$O(F m)$` でアルゴリズムが終了することも言える.

さて, 辺容量の平均が `$k$` であるとする.
primal step で発見したパスに属する各辺からは, 残余容量が少なくとも `$1$` 減らされる. 残余容量の合計は `$O(k m)$` であるから, 各 primal step は `$O(k m + m) = O(km)$` である.

一方, `$l \in \set{1,\dots,n-1}$` を任意にひとつ選び, `$l \ge \label(t)$` であるような最初の dual-primal step が開始した時点を考える.
`$V_i = \set{ v \setmid \label(t) = i}, W_i = \set{ v \setmid \label(t) \le i } = \cup_{j=0}^{i} U_j$`, とすると, 残余容量が正であるような辺 `$(u, v)$` は `$\label(u) + 1 \ge \label(v)$` であるから, `$W_i$` による残余カット容量は `$V_i$` と `$V_{i+1}$` の間の辺の残余容量の和である. 従って, `$W_i$` による残余カット容量の(`$i$` を動かしたときの)和は辺容量の和以下, すなわち高々 `$km$` である.
`$W_i$` は `$0 \le i < l$` で `$s$`-`$t$` カットを与えるから, この `$l$` 個のカットの残余カット容量の最小値は `$km / l$` 以下である.

以上から, 各辺の容量が高々 `$k$` であるとき, `$l \ge \label(t)$` であるような最初の dual-primal step に至るまでにかかる時間計算量は `$O(kml)$` であり, この後最大流に至るまでの時間計算量は `$O((km / l) m)$` であるから, 合計で `$O(kml + (km/l) m)$` であり, `$l = \sqrt{m}$` とすれば `$O(k m^{3/2})$` であることがわかった.

更に, 各辺の容量が高々 `$k$` で, 多重辺が無いとする. `$0 \le i < l$` に対する `$V_i$` を, `$(V_0, V_1), (V_2, V_3), \dots$` のようにペアに分割すると, 少なくとも1つのペア `$(V_i, V_{i+1})$` の要素数の合計が `$O(2n / l) = O(n / l)$` である. 従って, このペア間の辺の残余容量の合計, すなわち `$W_i$` による残余カット容量は `$O(k (n / l)^2)$` である.
よって, 各辺の容量が高々 `$k$` で多重辺が無いとき, Dinic 法が `$O(kml + k (n/l)^2 m)$` であり, `$l = n^{2/3}$` とすれば `$O(k n^{2/3} m)$` であることがわかった.


### 頂点容量の平均値が高々定数

頂点の容量を, その頂点へ入る辺の容量の和と, その頂点を出る辺の容量の和の小さい方とし, その(頂点を動かした時の)平均値を `$k$` とする. すなわち,
`$$k = \operatorname{avg}_v \min(\sum_{e \in \delta^+(v)} u_e, \sum_{e \in \delta^-(v)} u_e)$$`
とする. 前節と同様, `$l = \label(t)$` のときの `$V_i$` や `$W_i$` を考える.
`$$
A_i = \set{ v \in V_i \setmid \sum_{e \in \delta^+(v)} u_e < \sum_{e \in \delta^-(v)} u_e }, \quad B_i = V_i \setminus A_i
$$`
とし, `$U_i = W_{i-1} \cup A_i$` とすると, `$U_i$` による残余カット容量は `$\sum_{e \in \delta^+(A_i)} u_e + \sum_{e \in \delta^-(B_i)} u_e$` で上から抑えられる. これの(`$i$` を動かしたときの)和は高々 `$kn$` だから, 少なくとも一つの `$i$` で `$U_i$` による残余カット容量が高々 `$kn / l$` になる.

よって, 各頂点の容量が高々 `$k$` であるとき, Dinic 法が `$O(kml + k(n/l)m)$` であり, `$l = \sqrt{n}$` とすれば `$O(k \sqrt{n} m)$` であることがわかった.

二部グラフ `$(U, V)$` 上の二部マッチングを, 頂点 `$s$`, `$t$` と `$s$` から `$u \in U$`, `$v \in V$` から `$t$` への辺を加えたグラフ上で, 辺容量 `$1$` として最大流問題に帰着する場合, `$s$` へ入る辺, `$t$` から出る辺の容量和はそれぞれ `$0$` であり, `$u \in U$` へ入る辺, `$v \in V$` から出る辺の容量和は `$1$` であるから, 上で `$k = 1$` とした場合の計算量 `$O(\sqrt{n} m)$` を得る.


### 例外がある場合

上の証明は, 辺容量や頂点容量が `$k$` を超えたり, 多重辺がある場合でも, そのような頂点/辺が高々有限個であるときは, ちょっと変更すればオーダーとしては同じものが得られる.
定数個の, 条件を満たさない超頂点を加えた場合などに役に立つだろう.


### GCD

容量に関しては, min と加減算程度しか使わないため, 容量の最大公約数 `$g = \operatorname{GCD}_{e} u_e$` で各容量を割ってもアルゴリズムの挙動は変わらない.
これを用いると, 例えば容量が `$1$` とは限らない定数の場合でも, `$k=1$` として上の結果を利用してよいことがわかる.

### 無向基礎グラフにおける `$s$`-`$t$` 最長路長が短い場合

これについては容量の整数性を仮定しなくともよい. 無向基礎グラフ(もしくは `$\exG$`)上での `$s$`-`$t$` 最長路長を `$l$` とすると, 各 dual-primal step で `$\label(t) \le l$` であるから, dual-primal step は高々 `$l$` 回しか行なわれない.

dual step は `$O(m)$` である.
一方 primal step の計算量は, 発見するパスの長さと `$\ce(\cdot)$` の更新回数の和に関する線形時間であった.
ここで, パスの長さは `$O(l)$` でパスは各 primal step で高々 `$m$` 本見つかり, `$\ce(\cdot)$` の更新回数は一つの primal step につき `$O(m)$` であったから, 全て合わせて `$O(l (m + l m + m) ) = O(l^2 m)$` である.

特に, `$G$` の無向基礎グラフにおいて頂点集合を二つの独立集合 `$A$` と `$B$` に分割出来る(すなわち二部グラフである)場合, `$s = \min(|A|, |B|)$` とすると, `$s$`-`$t$` 最長路長は高々 `$2 s$` であり, そのグラフ上での Dinic 法の計算量は `$O(s^2 m)$` となる.


### 余談

上の証明を見ていればわかるように, 多くのフローが序盤の dual-primal step で流れる事が期待される.
簡単なインスタンス, 例えばランダムグラフは, 経験的に, この性質を強くもつものが多い. [^myimpression]
一方で, 計算量には, パスが見つかった際に流すフローで, そのパスのどれだけの辺を消滅させられるかも関わってくる.
こちらは, 上の容量が高々定数な場合のように, フロー空間が縮退しているインスタンスが簡単になる.

[^myimpression]: 個人の感想です.

## 参考文献

- [R.K.Ahuja, T.L.Magnanti, J.B.Orlin, "Network Flows: Theory, Algorithms, and Applications", Prentice-Hall, Inc., 1993.][netbook]

[netbook]: https://www.amazon.co.jp/dp/013617549X "Network Flows: Theory, Algorithms, and Applications"
