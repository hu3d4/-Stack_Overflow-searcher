# Stack Overflow Searcher

Stack Overflow searcher は検索ボックスに入力された文字列を Google 検索し、そこから Stack Overflow のサイトだけを抽出して表示します。

注意: 使用するブラウザは chrome を想定して作られています。
ページをリロードしないと履歴が反映されない場合があります。

![Screenshot from 2021-06-10 22-04-08](https://user-images.githubusercontent.com/66501033/121530031-f48faa00-ca37-11eb-80b9-efdaa910b053.png)

-

![Screenshot_2021-02-02 Programmable Search Engine](https://user-images.githubusercontent.com/66501033/106545995-c3651180-654d-11eb-8bf5-7dd72a642b88.png)

# 機能一覧

- 検索機能
- 検索履歴の保存
- 履歴からの補完機能
- 履歴一括削除
- 特定の履歴を一つだけ削除する機能
- ログインログアウト機能

# 使用技術

- Rust 1.49
- Actix Web 3.3.2
- Diesel 1.4.1
- PostgreSQL 13.1
- Docker-compose 1.28.2
- AWS
  - VPC
  - EC2
- Programmable Search Engine by Google
