# Stack Overflow Searcher

Stack Overflow searcherは検索ボックスに入力された文字列をGoogle検索し、そこからStack Overflowのサイトだけを抽出して表示します。

普通にGoogleで検索するとSEOの強い記事がヒットして、本当にほしい情報を見つけることが大変なときがあります。
そんなときにこれを使えば英語のStack Overflowのサイトだけを表示してくれるのでプログラミング学習を円滑に進めることがきでます。


注意: 使用するブラウザはchromeを想定して作られています。
      ページをリロードしないと履歴が反映されない場合があります。

![Screenshot_2021-02-08 Stack Overflow Searcher](https://user-images.githubusercontent.com/66501033/107167188-87bdc200-69fb-11eb-9ccb-6cede2199372.png)

--------------------------------------------------------------------------------------------------------------------------

![Screenshot_2021-02-02 Programmable Search Engine](https://user-images.githubusercontent.com/66501033/106545995-c3651180-654d-11eb-8bf5-7dd72a642b88.png)

# 機能一覧
* 検索機能
* 検索履歴の保存
* 履歴からの補完機能
* 履歴一括削除
* 特定の履歴を一つだけ削除する機能
* ユーザー判別機能

# 使用技術
* Rust 1.49
* Actix Web 3.3.2
* Diesel 1.4.1
* PostgreSQL 13.1
* Docker-compose 1.28.2
* AWS
    * VPC
    * EC2
* Programmable Search Engine by Google
