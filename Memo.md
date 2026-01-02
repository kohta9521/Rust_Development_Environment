# Rust × モバイルバックエンド学習ロードマップ

最初は **Rust の基礎 → Web API → DB → モバイル連携** の順で進めるのがおすすめ。[web:20]

---

## 全体の進める順番

1. Rust 基礎（The Rust Programming Language 日本語版 第 1〜6 章）[web:20]
2. Axum でシンプルな Web API（`GET /health`, `GET /hello`）[web:40]
3. Rust + SQLite で DB 入門（テーブル作成・SELECT/INSERT）[web:19]
4. Axum + DB を繋いで「API から DB を読む/書く」[web:19]
5. React Native から Rust API を叩いて画面に表示
6. 余力があれば Postgres/Supabase や、Rust でモバイルロジック共有の検討[web:39]

---

## ステップ 1：Rust 基礎

- やること
  - The Rust Programming Language 日本語版の第 1〜6 章を読む[web:20]
  - `cargo new` で小さい CLI ツール（じゃんけん・ToDo など）を 1 つ作る
- 注意点
  - 所有権・借用・ライフタイムは一度で理解しようとせず、第 4 章は 2 周前提で読む[web:35]
  - 公式本で詰まった箇所だけ Qiita や Zenn の記事など日本語情報で補う[web:24]

---

## ステップ 2：Axum で Web API

- やること
  - `cargo new backend` → Axum と Tokio を導入
  - `GET /` で `"Hello, World!"` を返す最小サーバーを作る[web:40]
  - `GET /health` と `GET /hello?name=xxx` を追加し、JSON レスポンスも試す[web:43]
- 注意点
  - 最初はルーティング＋ハンドラ＋ Json 返却だけに絞り、認証やミドルウェアには手を出さない[web:43]
  - ポート番号や環境をあまり弄らず、開発フローをシンプルに保つ

---

## ステップ 3：Rust + SQLite で DB

- やること
  - `sqlx` もしくは `rusqlite` で SQLite ファイルに接続する[web:19]
  - `users` / `encounters` など簡単なテーブルを作成
  - 小さな main 関数から `INSERT` / `SELECT` を実行して動作確認[web:38]
- 注意点
  - いきなり非同期＋ DB を同時にやると難度が上がるので、まずは同期的なサンプルから始める[web:19]
  - スキーマは「とりあえず動く」を優先し、あとでリファクタ前提で軽めに設計する

---

## ステップ 4：Axum + DB をつなぐ

- やること
  - Axum のハンドラから SQLite/Postgres に接続し、`GET /users` で DB の中身を JSON 返却[web:19]
  - `POST /users` で JSON を受け取り、DB に INSERT するエンドポイントを 1 つ作る[web:43]
- 注意点
  - コネクションプール（例: `sqlx::Pool`）をアプリ全体で共有する設計にする[web:19]
  - エラーハンドリングは最初から凝らず、`Result`で返してログを見る程度に留める[web:43]

---

## ステップ 5：モバイル（React Native）から Rust API を叩く

- やること
  - 開発環境では `http://localhost:3000` のような URL で Rust API を起動
  - React Native から`fetch`で `GET /users` や `GET /tracks` を叩き、FlatList などで表示
- 注意点
  - 実機/エミュレータでは `localhost` ではなく PC の IP アドレス＋ポートを使う必要がある
  - CORS や HTTP/HTTPS の違いなどネットワーク系の罠を 1 つずつ確認しながら進める

---

## ステップ 6：発展（Supabase・モバイル Rust・書籍）

- やること候補
  - SQLite から Postgres/Supabase に差し替えて本番寄りの構成にする[web:19]
  - Rust でコアロジックを書き、モバイルやバックエンドと共有するアーキテクチャの記事を読む[web:39]
  - 「プログラミング Rust 第 2 版」などの商業本で非同期処理や高度なトピックを補強[web:42]
- 注意点
  - いきなり全部を Rust で統一せず、まずは 1 機能だけ Rust バックエンドで試す
  - 個人開発では学習とプロダクト進行のバランスを意識し、深追いしすぎないようにする
