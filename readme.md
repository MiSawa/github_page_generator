# 起動とか

Docker を入れる.
`./docker-build.sh` で dockar image を作る.
`./run.sh` で起動し, [localhost:4567](localhost:4567) でプレビュー.
`./build.sh` で git submodule である `MiSawa.github.io/` にビルドされる.
`./shell.sh` で docker container の中で bash を開く.


# front matter に書くこと

    problem:
        round: SRM650
        level: [Div1Easy, Div1Medium, Div1Hard]
        rd: 16314
        pm: 13669
        name: TaroFillingAStringDiv1
        url: http://community.topcoder.com/stat?c=problem_statement&pm=13669&rd=16314
    date: 2015/03/03
    tags: [TopCoder, 数え上げ, 算数]
    source_code: ./src/AC_TaroFillingAStringDiv1.cc

とか. ``source_code`` は省略すると見に行かない.
``date`` は解いた日.
``state`` で ``AC`` か ``WA`` かとかも入れるかなぁ.

