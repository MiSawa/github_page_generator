---

title: 最短経路問題に対するコストスケーリング法
date: 2021/02/05
tags: [フロー, 最短路]
secret: false

---
<div style="display:none">$$
\gdef\epsilon{\varepsilon}
\gdef\floor#1{\lfloor {#1} \rfloor}
\gdef\ceil#1{\lceil {#1} \rceil}
\gdef\epsceil#1{\lceil {#1} \rceil_\epsilon}
$$</div>

# 最短経路問題に対するコストスケーリング法

## 概要

`$n$` 頂点 `$m$` 辺のグラフと(非負とは限らない)辺費用 `$c_e$` が与えられる.
この入力での最短経路問題は Bellman-Ford 法や SPFA で `$O(nm)$` で計算可能であることがよく知られている.
この記事では, 辺費用が `$-C$` 以上の整数である場合に適用できる `$O(\sqrt{n} m \log(C))$` のアルゴリズムを[この論文][paper]より紹介する.
このアルゴリズムの一種は最小費用流問題に対するコストスケーリング法の最適化で, 具体的にはポテンシャルのみを変更することで scaling phase を飛ばすことが出来るか判定するために使われることがある.
個人的には, [Dilworthの定理][Dilworth]/[Mirskyの定理][Mirsky]の系を活用しているあたりが非常に面白いと思った.


## 準備

この節では, 後に必要となる chain と antichain を定義し, その大きさについての定理を示す.

`$n$` 頂点の DAG (Directed Acyclic Graph, 有向非巡回グラフ) `$G$` を考える.
頂点 `$u$` から頂点 `$v$` への(空でない)パスが存在するとき `$u \succ v$`, そうでないとき `$u \not\succ v$` と書くことにする.
頂点部分集合 `$S$` であって, 任意の相異なる2頂点 `$u, v \in S$` が `$u \succ v$` 又は `$v \succ u$` を満たすものを **chain (鎖)** と呼ぶ.
また, 頂点部分集合 `$S$` であって, 任意の相異なる2頂点 `$u, v \in S$` が `$u \not\succ v$` かつ `$v \not\succ u$` を満たすものを **antichain (反鎖)** と呼ぶ.
例えば, `$G$` のパスの頂点集合は chain であり, 入次数 `$0$` の頂点全体からなる集合は anti chain である.

chain と antichain の要素数の意味での最大値に関して, 次の定理が成り立つ:

**定理1**:

:::indent
任意の `$n$` 頂点の DAG `$G$` に対し, `$G$` の chain 又は antichain で要素数が `$\sqrt{n}$` 以上のものが存在する.[^Dilworth]
:::

[^Dilworth]: これは [Dilworth の定理][Dilworth]や [Mirsky の定理][Mirsky]からも導ける.

**証明**:

:::indent
`$G$` に新たに頂点 `$s$` を加え, `$s$` から他の全ての頂点へ辺を張る.
すると, `$G$` は DAG のままであり, `$s$` でない任意の頂点 `$v$` に対し `$s \succ v$` となる.
`$G$` の各頂点 `$v$` に対し, `$s$` からの最*長*経路長を `$l_v$` とする.[^longest_path]
`$l$` の最大値が `$\sqrt{n}$` 以上であるとき, 最大値をとる頂点の一つを `$v$` とすると, `$s$` から `$v$` へのパスで頂点数が `$\sqrt{n}+1$` 以上のものが存在し, その頂点集合から `$s$` を除いたものが元の DAG の chain になっている.
一方 `$l$` の最大値が `$\sqrt{n}$` 未満であるとき, `$s$` を除く頂点での値は `$1, 2, \dots, \sqrt{n}-1$` の高々 `$\sqrt{n}-1$` 通りである.
鳩ノ巣原理から, `$l$` の値が等しくなる `$\lceil n / (\sqrt{n}-1) \rceil \ge \sqrt{n}$` 個の(`$s$` でない)頂点が存在し, これは元の DAG の antichain になる.
実際, `$u \succ v$` であるならば, `$s$` から `$u$` へ任意のパスに対して `$u$` から `$v$` へのパスを繋げることで `$s$` から `$v$` へのパスを作れるから `$l_u < l_v$` であり, この対偶を用いて `$l_u = l_v$` なら `$u \not\succ v$` かつ `$v \not\succ u$` であることが示される. ∎
:::

[^longest_path]: `$s \succ v$` だから経路が存在し, `$G$` が DAG だから経路は有限通りで最長のものが取れる.


**応用**

:::indent
例えば, 相異なる `$n$` 個の要素を持つ数列 `$a_1, \dots, a_n$` に対し, `$1, \dots, n$` を頂点とし `$i < j$` かつ `$a_i < a_j$` な `$(i, j)$` を辺とする DAG を考える.
すると, 数列 `$\{a_i\}$` の単調増加部分列と chain, 単調減少部分列と antichain が一対一対応する.
従って, `$\sqrt{n}$` 個以上の要素を持つ部分列であって, 単調増加または単調減少なものが存在することがわかる. [^ErdosSzekeres]
:::

[^ErdosSzekeres]: これをもう少し精細にしたものが [Erdős–Szekereの定理][ErdosSzekeres] である.


## 問題設定と用語の準備
この辺りのことは [ながたかなさんの記事](https://qiita.com/ngtkana/items/d7fc4463e56b966d1ebf) にも詳しく書かれているので, ポテンシャルや被約費用を扱ったことがない人は一読するとよいだろう.

`$n$` 頂点 `$m$` 辺の単純[^simple]有向グラフ `$G = (V, E)$` と, 辺費用 `$c(e) \in \Z$` が与えられる.
頂点から実数への関数 `$\pi: V \to \R$` をポテンシャルと呼ぶ.
ポテンシャル `$\pi$` に対し, `$c^\pi((u, v)) := c((u, v)) + \pi(u) - \pi(v)$` を **被約費用 (又は簡約費用, reduced cost)** と呼ぶ.[^reduced_cost_sign]
任意の辺に対して被約費用が非負であるとき, そのポテンシャルは**feasible (実行可能)** であると言う.

[^simple]: 辺を `$(u, v)$` と簡便に表記したいだけで, 本質的な要請ではない.
[^reduced_cost_sign]: 文献によっては, `$c^\pi((u, v)) := c((u, v)) - \pi(u) + \pi(v)$` としていることもある. そちらに合わせたい場合は, ポテンシャルに関する符号を全て逆転させて読みかえればよい.

任意の `$s$`-`$t$` パス `$P$` と閉路 `$C$` に対し,
`$$
\begin{aligned}
c(P) &:= \sum_{e \in P} c(e),
&c^\pi(P) &:= \sum_{e \in P} c^\pi(e), \\
c(C) &:= \sum_{e \in C} c(e),
&c^\pi(C) &:= \sum_{e \in C} c^\pi(e)
\end{aligned}
$$`
とすると, `$c^\pi$` の定義から,
`$$
\begin{aligned}
c^\pi(P)
&= \sum_{(u, v) \in P} \left( c((u, v)) + \pi(u) - \pi(v) \right) \\
&= c(P) + \pi(s) - \pi(t), \\ \\
c^\pi(C)
&= \sum_{(u, v) \in C} \left( c((u, v)) + \pi(u) - \pi(v) \right) \\
&= c(C)
\end{aligned}
$$`
である.
従って, 閉路の被約費用はポテンシャルのとり方に依らない. また, パスの被約費用は, そのパス自体の費用, 始点と終点でのポテンシャルの値のみに依存する.
よって, `$s$`-`$t$` 最短経路は, その辺費用を `$c$` としても `$c^\pi$` としても変わらない.

ポテンシャル `$\pi$` が feasible であるとき, その定義より `$c^\pi(e) \ge 0$` であるから, 非負費用 `$c^\pi$` による最短経路問題を解けばもとの費用 `$c$` での最短経路問題と同じ解を得ることが出来る.
非負費用であるとき, 最短経路問題は Dijkstra 法で `$O(m + n \log n)$` で解けることが知られているから, feasible なポテンシャルを高速に求めるアルゴリズムが欲しい.

上の議論では, feasible なポテンシャルの存在については言及していなかった.
例えば負閉路, すなわち `$c(C) < 0$` となる閉路 `$C$` があるとき, 任意のポテンシャル `$\pi$` には, `$c(C) = c^\pi(C) = \sum_{e \in C} c^\pi(e) < 0$` であるから `$c^\pi(e) < 0$` となる辺が存在し, feasible でない.
しかし, この時はそもそも最短経路が存在しない[^shortest_path_existence].
一方, 負閉路が無ければ feasible なポテンシャルが存在する.
実際, `$G$` に頂点 `$s$` を加え, `$s$` 以外の任意の頂点へ `$s$` から費用 `$0$` の辺を張ったグラフを考える. `$s$` から `$v$` への最短距離を `$\pi(v)$` とすると, 任意の `$G$` の辺 `$e = (u, v)$` に対し, `$\pi$` の最適性から `$\pi(v) \le c(e) + \pi(u)$` であるが, これは `$\pi$` が feasible なポテンシャルであることに他ならない.

[^shortest_path_existence]: 正確には, `$s$` から到達可能かつ `$t$` へ到達可能な負閉路があるとき, `$s$`-`$t$` 最短経路が存在しない. この条件は, 例えば前処理としてグラフを強連結成分分解し, 各強連結成分で負閉路の存在判定をした後, 負閉路のない成分のみを残したグラフを考えることで落とすことが出来る.

以下の節での目的は, feasible なポテンシャル, すなわちポテンシャル `$\pi: V \to \R$` であって, 任意の辺 `$e = (u, v) \in E$` で `$c^\pi(e) = c(e) + \pi(u) - \pi(v) \ge 0$` となるものが存在するか判定し, 存在する場合は実際に構成するアルゴリズムを作ることである.


## 緩和とスケーリング法

feasible なポテンシャルの定義は `$\forall e \in E, c^\pi(e) \ge 0$` であった.
`$\epsilon > 0$` でこれを緩和した `$\forall e \in E,\ c^\pi(e) > -\epsilon$` を満たすポテンシャル `$\pi$` を **`$\epsilon$`-feasible (`$\epsilon$`-実行可能)** であるという. [^eps_feasibility_def]
また, `$c^\pi(e) \le 0$` である辺を **admissible (使用可能, 又は許容)** であると言い, `$c^\pi(e) \le -\epsilon$` である辺を **`$\epsilon$`-improvable な辺** であると言う. [^admissible_def] また, `$\epsilon$`-improvable な辺 `$(u, v)$` が存在するような頂点 `$v$` を **`$\epsilon$`-improvable な頂点** であると言う.

入力の整数性と前節の feasible なポテンシャルの構成から, feasible なポテンシャルが存在することと feasible な整数ポテンシャルが存在することは同値であるから, 以下補足の節まで, ポテンシャルは整数ポテンシャルのみを考える.
すると, `$\epsilon = 1$` のとき, `$\epsilon$`-feasible なポテンシャル `$\pi$` は `$c^\pi((u, v)) = c((u, v)) + \pi(u) - \pi(v) > -1$` を満たすが, 左辺は整数であるから `$c^\pi(e) \ge 0$`, つまり feasible である.

[^eps_feasibility_def]: `$c^\pi(e) \ge -\epsilon$` で定義する場合もあり, 後の議論が変わる. 詳細は補足の節で見る.
[^admissible_def]: `$c^\pi(e) \ge -\epsilon$` で `$\epsilon$`-feasibility を定義した場合は, admissible edge は `$c^\pi(e) < 0$`, improvable edge は `$c^\pi(e) < -\epsilon$` で定める.

さて, `$C = \max(2, \max_e(-c_e))$` とする. 従って, `$c_e \ge -C$` かつ `$\log C > 0$` である[^log_base].
`$\epsilon = 2^{\floor{\log C} + 1}$` とすると, `$c_e = c_e + 0 - 0 \ge -C > -\epsilon$` であるから, ゼロポテンシャル `$\pi(\cdot) = 0$` は `$\epsilon$`-feasible である.
コストスケーリング法は, この `$\epsilon$` とゼロポテンシャルを初期値とし, より小さい `$\epsilon$` に対する `$\epsilon$`-feasibility を満たすよう `$\pi$` を変更していく, 次のようなアルゴリズムである.

[^log_base]: `$\mathrm{log}$` の底は `$2$` とする.

1. `$\epsilon = 2^{\floor{\log C} + 1}$`, `$\pi(\cdot) = 0$` とする.
2. 不変条件: `$\pi$` は `$\epsilon$`-feasible で, `$\epsilon$` の整数倍のみを値に持つポテンシャルである.
3. `$\epsilon = 1$` であるならば終了し, そうでないならば `$\epsilon := \epsilon / 2$` と更新する.
4. `$\operatorname{Refine}(\epsilon, \pi)$` を呼び出す. この Refine は, `$(2\epsilon)$`-feasible なポテンシャルを `$\epsilon$`-feasible なポテンシャルに変形するか, その非存在を報告して(アルゴリズム全体を)終了するサブルーチンである.
5. 2.へ戻る.

Refine は `$O(\log C)$` 回呼び出されるから, Refine の時間計算量が `$O(f(n, m))$` であるならば, コストスケーリング法全体の時間計算量は `$O(f(n, m) \log C)$` である.
以下, この Refine の実現を目標とする.


## Admissible graph と Decycle
ポテンシャル `$\pi$` を一つ固定する. このポテンシャルについて Admissible な辺 `$E^\pi := \set{e \in E \setmid c^\pi(e) \le 0}$` のみを取り出したグラフ `$G^\pi = (V, E^\pi)$` を **admissible graph** と呼ぶ.
この `$G^\pi$` の任意の閉路 `$C$` のコストは `$c(C) = c^\pi(C) = \sum_{e \in C} c^\pi(e) \le 0$` であり, `$C$` が負の被約費用を持つ辺を含むならば `$C$` は負閉路である.
従って, `$G^\pi$` に両端点が同じ強連結成分に属する辺で負の被約費用を持つものが存在するかを判定し, 存在するならばアルゴリズムを終了してよい.

一方, そのような辺が存在しない場合を考える.
両端点が同じ強連結成分に含まれる辺全体の集合を `$S$` とする.
`$G$` をもとにし, 強連結成分たちを縮約したグラフ `$H$` を考えると[^decycle_details], `$H$` 上のポテンシャル `$\rho$` と `$G$` 上の `$S$` に含まれる辺の被約費用が `$0$` であるようなポテンシャル `$\rho'$` は, その整数性, feasibility や `$\epsilon$`-feasibility を保ったまま 1:1 対応し, `$H$` の `$\pi$` に対応するポテンシャルでの admissible graph は DAG になる.
従って, 必要ならば強連結成分分解をすることで, admissible graph は DAG であるとしてよい. この操作を decycle と呼ぶ. [^decycle_not_needed]

[^decycle_details]: 辺の費用などは適切に調節する. LP 的には, この強連結成分は, 双対の制約式で組み合わせることで等号が成立することがわかる集合 (例えば `$x-y \le 0$` と `$y-x \le 0$` など) に対応している. この等式を使って変数を削ぎ落としていくのが decycle である.
[^decycle_not_needed]: `$\epsilon$`-feasibility の定義を変えると, decycle が必要なくなる一方で, 別の面倒が発生する. 詳しくは補足の節で.


## Cut-Relabel
さて, `$(2\epsilon)$`-feasible なポテンシャル `$\pi$` と, (必要ならば decycleし) 付随する DAG である admissible graph `$G^\pi$` を考える.
頂点集合 `$S$` に対し, `$G^\pi$` 上で `$S$` から辿り着ける頂点全体を `$S$` の閉包と呼び, `$\overline{S}$` で表す. `$S = \overline{S}$` であるとき, `$S$` は閉集合であるという. 任意の頂点集合 `$S$` に対し, `$\overline{S}$` は閉集合である.

`$S$` が閉集合であるとき, その要素に対応する `$\pi$` を `$\epsilon$` 減少させる操作を `$\operatorname{CUT\_RELABEL}(S)$` で表す.
この操作により, `$c^\pi((u, v)) = c((u, v)) + \pi(u) - \pi(v)$` は `$u \not \in S$` かつ `$v \in S$` のとき `$\epsilon$` 増加し, `$u \in S$` かつ `$v \not \in S$` のときに `$\epsilon$` 減少する.
しかし `$S$` の選び方から, `$u \in S$` かつ `$v \not \in S$` であるような辺 `$e = (u, v)$` は admissible でないから, この操作の後も `$c^\pi(e) > 0 - \epsilon = - \epsilon$` を満たす.
従って, この操作は `$\epsilon$`-improvable な辺や頂点を増やさない.
また, 更新前の `$\pi$` が `$\epsilon$` の整数倍であるならば更新後も `$\epsilon$` の整数倍である.

どの二頂点間にも最後に `$\epsilon$`-improvable な辺を通るパスが存在しないような頂点部分集合をひとつ取り, `$X$` とする.
`$e = (u, v)$` が `$v \in X$` かつ `$\epsilon$`-improvable であるとすると, `$X$` と閉包の定義から `$u \not \in \overline{X}$` である.
`$\pi$` が `$(2\epsilon)$`-feasible であったことを思い出すと, この辺は `$\operatorname{CUT\_RELABEL(\overline{X})}$` によって `$c^\pi(e) > -2\epsilon + \epsilon = - \epsilon$` となり, `$\epsilon$`-improvable でなくなることがわかる.
よって, このような頂点集合 `$X$` に含まれる `$\epsilon$`-improvable な頂点は, `$\operatorname{CUT\_RELABEL}(\overline{X})$` によって `$\epsilon$`-improvable でなくなる.

これにより, Refine の実現として, 次のアルゴリズムが考えられる.

1. 必要ならば Decycle する.
2. `$\epsilon$`-improvable な頂点が存在しない, つまり `$\pi$` が `$\epsilon$`-feasible であれば, 終了する. そうでないとき, `$\epsilon$`-improvable な頂点をひとつ取り `$x$` とする.
3. `$\operatorname{CUT\_RELABEL(\overline{\set{x}})}$` を実行する
4. 1.へ戻る

上のアルゴリズムで, 2. を実行する度に `$\epsilon$`-improvable な頂点は減っていくから, 2. は高々 `$n$` 回実行される.
`$\epsilon$`-Improvable な頂点の発見や `$\operatorname{CUT\_RELABEL(\overline{\set{x}})}$` の実行は `$O(m)$` でできるから, 上の Refine は `$O(nm)$` であり, コストスケーリング法全体の時間計算量は `$O(nm \log C)$` となる.


## Eliminate-Chain
ポテンシャル `$\pi$` は `$2 \epsilon$`-feasible であり, その値は `$\epsilon$` の整数倍であるとする.
任意の `$x$` に対し, `$x$` 以上の最小の `$\epsilon$` の整数倍を `$\epsceil{x}$` で表す. すなわち, `$\epsceil{x}=\ceil{x/\epsilon}\epsilon$` とする. [^epsceil]
また, `$c^\pi_\epsilon(e) = \epsceil{c^\pi(e)}$` とする. 例えば `$c^\pi_\epsilon(P) = \sum_{e \in P} \epsceil{c^\pi(e)}$` である.

[^epsceil]: 標準的な記法をご存知の方, 教えてください!!

`$P$` を `$G^\pi$` のパスとする.
パスにこの順で含まれる頂点 `$u, v$` に対し, `$P_v$` をこのパスの始点から `$v$` までの部分パスとし, `$P_{u,v}$` を `$u$` から `$v$` までの部分パスとする. 他のパスに対しても同様の記法を使う. さて,
`$$
\rho(v) = \begin{cases}
c^\pi_\epsilon(P_v) &\text{if } v \in P, \\
0 &\text{otherwise}
\end{cases}
$$`
とすると, `$P$` は `$G^\pi$` のパスだったから, `$\rho(v) \le 0$` であり, `$\rho$` は `$P$` に沿って単調非増加である.
また, `$c^\pi(e) > -2\epsilon$` であるから, `$c^\pi_\epsilon(e) \ge -\epsilon$` であり, `$c^\pi_\epsilon(P_v) \ge - n \epsilon$` である.


`$G$` に新たな頂点 `$s$` を加え, `$s$` から他の全頂点に辺を加えたグラフを `$H$` とし, `$H$` 上の辺重み `$c'$` を
`$$
c'(e) = \begin{cases}
\rho(v) + n \epsilon & \text{if } e = (s, v), \\
\max\set{0, c^\pi_\epsilon(e)} & \text{otherwise}
\end{cases}
$$`
で定めると, これは非負であるから, `$s$` から他の全頂点への `$c'$` を辺コストとする最短経路長が存在する. これを `$d(v)$` とする.
更に, `$\pi'(v) = \pi(v) + d(v) - n \epsilon$` とする.
この `$\pi'$` によって `$\pi$` を更新するのが Eliminate-Chain である.

天下り的に定義したが, これは `$P$` に含まれる `$\epsilon$`-improvable な辺の終点について順に `$\operatorname{CUT\_RELABEL}(\overline{\set{x}})$` を行った結果を求めていると考えてよい.
実際 `$c^\pi_\epsilon(P_v)$` は `$P$` の始点から順に `$\epsilon$`-improvable な辺を通るごとに `$-\epsilon$` され, `$c'((s,v))$` はこれを表している. また, `$u$` が `$\max\set{0, c^\pi_\epsilon((u,v)) / \epsilon}$` 回 relabel されると辺 `$(u, v)$` は admissible になり, それ以降 `$u$` を relabel する度に `$v$` も relabel されることを `$c'(u, v)$` は表している.

さて, `$c'$` は `$\epsilon$` の整数倍であり, 最短経路長は `$d(v) \le c'((s, v)) = \rho(v) + n \epsilon \le n \epsilon$` を満たすから, Dial の方法[^dials_impl]による Dijkstra 法を実行することにより `$O(m)$` で計算可能である.
また, `$\pi$`, `$c'$` は `$\epsilon$` の整数倍であるから, `$d$`, 従って `$\pi'$` も `$\epsilon$` の整数倍である.

[^dials_impl]: ヒープの代わりに, priority をキーとするバケットに突っ込む方法. `$0$`-`$1$` BFS を知っている人は `$0$`-`$\cdots$`-`$n$` BFS と思ってもよい.

次に, `$\pi'$` は `$(2\epsilon)$`-feasible であり, 新たに `$\epsilon$`-improvable な辺が作られないことを示す.
任意の辺 `$e = (u, v)$` を取る.
`$d$` の最適性から `$c'(e) + d(u) - d(v) \ge 0$` であるから,
`$$
\begin{aligned}
c^{\pi'}(e)
&= c(e) + \pi'(u) - \pi'(v) \\
&= c(e) + (\pi(u) + d(u) - n \epsilon) - (\pi(v) + d(v) - n \epsilon) \\
&= c^\pi(e) + d(u) - d(v) \\
&= \left( c'(e) + d(u) - d(v) \right) + \left(c^\pi(e) - c'(e) \right) \\
&\ge c^\pi(e) - c'(e) \\
&= c^\pi(e) - \max\set{0, c^\pi_\epsilon(e)}
\end{aligned}
$$`
となる. `$e$` がもともと `$\epsilon$`-improvable であるとき
`$c^\pi(e) - \max\set{0, c^\pi_\epsilon(e)} = c^\pi(e) - 0 > - 2 \epsilon$`
であり, `$e$` がもともと `$\epsilon$`-improvable でなければ
`$c^\pi(e) - \max\set{0, c^\pi_\epsilon(e)} = c^\pi(e) - \epsceil{c^\pi(e)} > - \epsilon$`
であるから, いずれにせよ `$c^{\pi'}(e) > -2 \epsilon$` であり, `$\pi'$` は `$\pi$` から `$\epsilon$`-improvable な辺を増やさないことがわかった.

**補題1**:

:::indent
`$G$` に負閉路が無いならば `$\forall v \in P,\ \pi'(v) = \pi(v) + \rho(v)$`.
:::

:::details **証明**:

:::indent
`$\pi'(v) = \pi(v) + d(v) - n \epsilon$` であるが, `$d$` の最適性から
`$d(v) \le c'((s, v)) = \rho(v) + n \epsilon$`.
従って `$\pi'(v) \le \pi(v) + \rho(v)$` である.

次に, `$\pi'(v) < \pi(v) + \rho(v)$` であると仮定し, `$G$` に負閉路が存在することを示す.
`$R$` を `$c'$`-最短 `$s$`-`$v$` パスとすると, 仮定から `$d(v) < c'((s, v))$` で, 双方 `$\epsilon$` の整数倍であるから
`$$
c'(R) = d(v) \le c'((s, v)) - \epsilon = \rho(v)+ n \epsilon - \epsilon.
$$`

`$R$` 上で `$s$` の次に訪れる頂点を `$w$` とすると, 仮定から `$w \neq v$` であり,
`$$
\begin{aligned}
c^\pi(R_{w,v})
& \leq c'(R_{w,v}) & (\because c^\pi \leq c') \\
& = c'(R) - c'((s, w)) &\\
&= c'(R) - (\rho(w) + n \epsilon) &\\
&\leq (\rho(v) + n \epsilon - \epsilon) - (\rho(w) + n \epsilon) & (\because \text{上の不等式})\\
&= \rho(v) - \rho(w) - \epsilon.&
\end{aligned}
$$`
一方, `$c'$` の非負性と上の不等式(の2番目と最後)から
`$0 \le c'(R_{w,v}) < \rho(v) - \rho(w)$`
であるが, `$\rho(v) \le 0$` だから, `$0 \ge \rho(v) > \rho(w)$`.
`$\rho$` の定義から `$w$` は `$P$` に含まれ, 更に `$\rho$` の単調性から, `$w$` は `$v$` 以降の位置にある.

`$$
\begin{aligned}
c^\pi(P_{v,w})
&\le c^\pi_\epsilon(P_{v,w}) \\
&= c^\pi_\epsilon(P_w) - c^\pi_\epsilon(P_v) \\
&= \rho(w) - \rho(v)
\end{aligned}
$$`
であるから, `$C$` を `$R_{w,v}$` と `$P_{v,w}$` をつなげた閉路とすると
`$$
\begin{aligned}
c^\pi(C)
&= c^\pi(R_{w,v}) + c^\pi(P_{v,w}) \\
&\le (\rho(v) - \rho(w) - \epsilon) + (\rho(w) - \rho(v)) \\
&< 0,
\end{aligned}
$$`
つまり `$C$` は負閉路である.∎
:::
:::


**定理2**:

:::indent
`$G$` に負閉路が無いとする.
`$P$` の始点でない頂点 `$v$` が `$\pi$` で `$\epsilon$`-improvable であり, `$P$` において `$v$` に入る辺が `$\pi$` で `$\epsilon$`-improvable であるならば, `$v$` は `$\pi'$` では `$\epsilon$`-improvable でない.
:::

:::details **証明**:

:::indent
`$v$` が `$\pi$` でも `$\pi'$` でも `$\epsilon$`-improvable であるとする.
`$\pi'$` で新たに `$\epsilon$`-improvable になる辺は無いから, `$\pi'$` でも `$\pi$` でも `$\epsilon$`-improvable な `$v$` に入る辺が存在する.
このような辺を任意に取り, `$e = (u, v)$` とする.

`$Q$` を `$s$` から `$u$` への `$c'$`-最短パスとし, `$Q$` で `$s$` の次の頂点を `$w$` とする.

`$$
\begin{aligned}
-\epsilon
&\ge c^{\pi'}(e) &(\because e \text{は} \pi' \text{で} \epsilon\text{-improvable}) \\
&= c(e) + \pi'(u) - \pi'(v) &\\
&= c(e) + (\pi(u) + d(u) - n \epsilon) - (\pi(v) + d(v) - n \epsilon) &\\
&= c^\pi(e) + d(u) - d(v) &
\end{aligned}
$$`
であるが, `$c^\pi(e) > -2 \epsilon$` であるから,
`$$
\begin{aligned}
d(u) - d(v)
&= c^{\pi'}(e) - c^\pi(e) \\
&\le - \epsilon - c^\pi(e) \\
&< -\epsilon + 2 \epsilon \\
&= \epsilon
\end{aligned}
$$`
であり, `$d$` は `$\epsilon$` の整数倍であったから, `$d(u) \le d(v)$`.

さて, `$P$` 上で `$v$` の一つ前の辺が `$\epsilon$`-improvable であった. この辺を `$(x, v)$` とすると,
`$$
\begin{aligned}
\rho(v) - \rho(x)
&= c^\pi_\epsilon(P_v) - c^\pi_\epsilon(P_x) \\
&= c^\pi_\epsilon((x, v)) \\
&\le - \epsilon
\end{aligned}
$$`
である. 従って,
`$$
\begin{aligned}
\rho(w)
&= c'((s, w)) - n \epsilon &(\because c' \text{の定義})\\
&= c'(Q) - c'(Q_{w, u}) - n \epsilon &\\
&\le d(u) - c'(Q_{w, u}) - n \epsilon &(\because Q \text{は最短路}) \\
&\le d(u) - n \epsilon & (\because c' \text{: 非負}) \\
&\le d(v) - n \epsilon &\\
&= (\pi'(v) - \pi(v) + n \epsilon) - n \epsilon & (\because \pi'\text{の定義}) \\
&= \rho(v) & (\because \text{補題1}) \\
&\le \rho(x) - \epsilon & (\because \text{上の不等式}) \\
&\le - \epsilon & (\because \rho \text{は非正})
\end{aligned}
$$`
であるから, `$\rho(w) \le \rho(v) < \rho(x) \le 0$` である.
この不等式と `$\rho$` の定義から, `$w \in P$` であり,
`$\rho$` の単調性から `$w$` は `$P$` 内で `$v$` 以降に出現する.

補題1から
`$$
\begin{aligned}
c^{\pi'}(P_{v,w})
&= c(P_{v,w}) + \pi'(v) - \pi'(w) \\
&= c(P_{v,w}) + (\pi(v) + \rho(v)) - (\pi(w) + \rho(w)) \\
&= c^\pi(P_{v,w}) + \rho(v) - \rho(w) \\
&= c^\pi(P_{v,w}) + c^\pi_\epsilon(P_v) - c^\pi_\epsilon(P_w) \\
&= c^\pi(P_{v,w}) - c^\pi_\epsilon(P_{v,w}) \\
&\le 0
\end{aligned}
$$`
となる. 一方,
`$$
\begin{aligned}
c^{\pi'}(Q_{w,u})
&= c^\pi(Q_{w,u}) + d(w) - d(u) & (\because \pi'\text{の定義}) \\
&\le c'(Q_{w,u}) + d(w) - d(u) & (\because c^\pi \le c') \\
&= 0 & (\because Q_{w,u} \text{は最短路, } d \text{は最短路長})
\end{aligned}
$$`
であり, これと `$c^{\pi'}(e) \le -\epsilon$` から, `$P_{v,w}$`, `$Q_{w,u}$`, `$e$` を繋げた閉路 `$C$` は
`$$
\begin{aligned}
c^{\pi'}(C)
&= c^{\pi'}(P_{v,w}) + c^{\pi'}(Q_{w,u}) + c^{\pi'}(e) \\
&\le 0 + 0 - \epsilon \\
&< 0,
\end{aligned}
$$`
すなわち負閉路である. ∎
:::
:::

**注意**:

:::indent
上の定理では, `$P$` の `$\epsilon$`-improvable な辺の指す先が `$\epsilon$`-improvable で無くなることは証明したが, 一般に `$\epsilon$`-improvable な頂点が `$\epsilon$`-improvable で無くなることは証明していない.
元の論文ではこれを証明している(と主張している).
しかし, どうにも "`$w \in P$`" と "`$w$` は `$v$` 以降に出現する" の二つの部分で行間を埋められず,
この記事ではこのような形にした.

他にも, `$P$` が `$\epsilon$`-improvable な辺から始まり, `$\epsilon$`-improvable な辺を最も多く含む `$G^\pi$` のパスであることを仮定すると, `$P$` に含まれる全ての `$\epsilon$`-improvable な頂点が `$\epsilon$`-improvable でなくなることが示せるだろう.
:::


さて, 上の定理から, `$\epsilon$`-improvable な辺一つからなるパスを `$P$` として Eliminate-Chain を繰返すことにより, `$O(nm)$` で Refine を実現出来ることがわかる.



## より高速な Refine の実現

Decycle した後の `$G^\pi$` に対し, その `$\epsilon$`-improvable な頂点全体を `$U$` とし, `$k = |U|$` とする.
頂点集合を `$U$` とし, `$u, v \in U$` に対し, `$u$` から `$v$` へのパスで最後に `$\epsilon$`-improvable な辺を通るものが存在するときに辺を張ったグラフを `$H$` とする.

`$G^\pi$` が DAG であったから `$H$` も DAG であり, 定理1 から `$H$` の chain 又は antichain で要素数が `$\sqrt{k}$` 以上のものが存在する.
`$H$` の chain `$S \subset U$` は, `$S$` を含む `$G$` のパス `$P$` で, `$S$` へ入る辺が `$\epsilon$`-improvable なものへ展開することができる.
一方 `$H$` の anti chain `$T \subset V$` は, どの二頂点間にも最後に `$\epsilon$`-improvable な辺を通るパスが存在しない.
従って, `$H$` の chain で要素数が `$\sqrt{k}$` 以上のものがあればそれに Eliminate-Chain, そうでなければ `$H$` の antichain で要素数が `$\sqrt{k}$` 以上のものに Cut-Relabel を適用することにより, `$O(m)$` で[^actually_linear]少なくとも `$\sqrt{k}$` 個の `$\epsilon$`-improvable な頂点を `$\epsilon$`-improvable でなくすことが出来る.

[^actually_linear]: `$H$` の構築と chain/antichain の発見に時間がかかりそうに見えるが, 実は陽に構築しないでも, `$G$` に超始点を加えたグラフで, `$\epsilon$`-improvable な辺を通る回数の意味での最長経路長を計算し, 定理1と同様に最長のパスか最大のバケットを取ればよい.

これを `$i$` 回繰り返した後の `$\epsilon$`-improvable な頂点の数を `$k_i$` とする.
`$k_i \ge k_0/2$` である間 `$k_i - k_{i+1} \ge \sqrt{k_0 / 2}$` であるから, このような `$i$` は高々 `$(k_0 / 2) / \sqrt{k_0 / 2} = \sqrt{k_0 / 2}$` 回しかない.
同様にして `$i$` 回目から `$O(\sqrt{k_i/2})$` 回後には少なくとも半減するから, 合計の実行回数は
`$$
O\left(\sum_{i=0}^{\infty} \sqrt{k_0 / 2^i}\right) = O(\sqrt{n})
$$`
である.

以上より, `$O(\sqrt{n} m \log C)$` のアルゴリズムが得られた.



## 補足1: feasibility, improvable, admissible などを別の定義にした場合

`$\epsilon$`-feasibility を `$c^\pi(e) \ge -\epsilon$` で定義することも出来る.
同時に, admissible edge は `$c^\pi(e) < 0$`, `$\epsilon$`-improvable edge は `$c^\pi(e) < -\epsilon$` で定める.
すると, admissible edge から `$=0$` が抜けたことにより, admissible graph は (decycle せずとも) DAG になる.

一方で, `$1$`-feasible な整数ポテンシャルであっても feasible とは限らなくなる.
しかし, `$\pi$` が `$\epsilon < \frac{1}{n}$` な `$\epsilon$` に対する `$\epsilon$`-feasible なポテンシャルであるとき, `$G$` の任意の閉路 `$C$` のコストは `$c(C) = c^\pi(C) = \sum_{e \in C} c^\pi(e) > n \cdot \frac{-1}{n} = -1$` であるから, 入力の整数性より非負である.
従って, feasible なポテンシャルが存在する.

以下, `$\epsilon < \frac{1}{n}$`, `$\pi$` は `$\epsilon$`-feasible なポテンシャルであるとし, ここから feasible なポテンシャルを得る方法を考える.

`$c'(e) = c^\pi(e) + \epsilon$` とすると, `$\pi$` の `$\epsilon$`-feasibility から `$c'(e) \ge 0$` である.
また, 任意のパス `$P$` に対し, `$0 \le c'(P) - c^\pi(P) < n \epsilon < 1$` であるから, `$c'$` での最短経路は `$c^\pi$` での最短経路に一致する.
従って, 超頂点 `$s$` と, `$s$` から各頂点へ `$c'((s, v)) = 0$` の辺 `$(s, v)$` を加えたグラフ上で `$s$` から各頂点への `$c'$`-最短経路木を求め, この最短経路木から `$s$` を除いた森の上での被約費用が `$0$` となるようポテンシャルを定めれば, これが feasible なポテンシャルになる.

従って, この方針では feasible なポテンシャルを合計 `$O(\sqrt{n} m \log(n C))$` で計算することが出来る.
実際には, 入力のコストを `$(n+1)$` 倍し, 常に整数に保つ実装にするとよい.


## 補足2: 実用上の話について

上では Refine の `$O(nm)$` の実装を二つと `$O(\sqrt{n} m)$` の実装を一つ紹介したが, 論文中には他にもいくつかの方法が与えられている.
おそらく実用的には, REFINE-P の方が速いのではないかと思う.
また, decycle, 特に縮約やそれを元に戻す操作をしなくてよいぶん, `$O(\sqrt{n} m \log(nC))$` の方が実測で速くなる可能性も高いだろう.


## 参考文献

- [Andrew V. Goldberg, 1993, "Scaling Algorithms for the Shortest Paths Problem", Proceedings of the fourth annual ACM-SIAM symposium on Discrete algorithms. Society for Industrial and Applied Mathematics, USA, 222-231.][paper]

[paper]: https://dl.acm.org/doi/10.5555/313559.313756
[Dilworth]: https://en.wikipedia.org/wiki/Dilworth%27s_theorem
[Mirsky]: https://en.wikipedia.org/wiki/Mirsky%27s_theorem
[ErdosSzekeres]: https://en.wikipedia.org/wiki/Erd%C5%91s%E2%80%93Szekeres_theorem
