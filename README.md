# はじめに
この`リポジトリ`は、私が`Rustの初心者の時に作成したコード`です。  
これからRustを学ぶ方の学習の助けになれば幸いです。  
※ 技術的ご質問にはお答えできません。

[SDL2 Sample Code!!!]  

<img src="pic/rust_sdl2.gif" alt="file">

# フィルダ構成
| ファイル、フィルダ | 説明       |
| ---------- | ---------- |
| `hello_world`  | "Hello, Rust!"と表示される      |
| `data_type`  | Rustのデータ型について      |
| `if_match`  | 条件分岐。if,matchを使用      |
| `loop_for_while`  |  繰り返し。loop,for,whileを使用      |
| `def_func`  | メソッド（関数）の宣言      |
| `rust_vec`  |  可変長さ配列の`Vec`の使い方      |
| `rust_generic`  |  `<T>`のジェネリック型      |
| `rust_vec_generic`  | `Vec`の使い方とジェネリック型      |
| `rust_opp`  | Rustでのオブジェクト指向      |
| `thread`  | Rustでの非同期スレッド      |
| `thread_channel`  | 非同期でのスレッド間のデータ送受信      |
| `thread_sync`  | 非同期でのスレッド間のデータ送受信      |
| `rust_tokio`  | tokioにグリーンスレッドでの非同期スレッド      |
| `file_io`  | `cargo run the doc/test.txt`でテキストを表示      |
| `rust_sdl2`  | SDL2による描画＆キーボード操作      |
  

# tokio 非同期I/Oクレート
数千～数百万のマルチスレッドでの非同期I/Oでは標準のstd::threadより、  
`tokio`を使用してグリーンスレッドで行うことを推奨します。

https://github.com/tokio-rs/tokio

## なぜ非同期I/Oにtokioのグリーンスレッドを推奨するのか？
※ 下記は`2000のグリーンスレッドで同期I/Oを実行`している例です。  
※ `tokio-consle`で確認しています  

<img src="pic/tokio_console.png" alt="a" width="50%">
  
標準のstd::threadは、Rustの標準ライブラリであり、OSのスレッドを使用して並行処理を行います。一方、tokio::taskは、Tokioと呼ばれる非同期ランタイムフレームワークの一部であり、グリーンスレッドを使用して非同期タスクを処理します。  
  
性能の観点では、一般的なケースではtokio::taskを使用したグリーンスレッドの方が高速です。これは、グリーンスレッドがスレッドの作成や切り替えのオーバーヘッドを削減できるためです。OSのスレッドを使用する場合、スレッドの作成や切り替えにはコストがかかりますが、グリーンスレッドは独自のスレッドスケジューリング機構を持ち、効率的な非同期処理を実現します。

ただし、性能は使用環境や具体的な処理内容に依存するため、一概にどちらが速いとは言えません。特定のケースや要件に応じて、適切なスレッドモデルを選択する必要があります。

また、注意点として、グリーンスレッドを使用する場合、非同期処理がI/Oバウンドであることが重要です。CPUバウンドな処理においては、std::threadや他のスレッドモデルの方が効果的です。

総括すると、tokio::taskを使用したグリーンスレッドは、一般的な非同期処理において高速かつ効率的ですが、具体的な状況に応じて最適な選択を行う必要があります。


# SDL2のインストール（Windows11 x64）
## 1.ダウンロード
下記URLのgithubから`SDL2-devel-2.x.x-VC.zip`をダウンロード  
https://github.com/libsdl-org/SDL/releases

## 2.インストール
解凍したファルダの`lib/x64` 内のファイルをすべて下記にコピー＆ペースト  
<img src="pic/sdl2_inst.png" alt="file" alt="a" width="80%">
  
[Windows11 x64の場合]  
>C:/Users/`{USER_NAME}`/.rustup/toolchains/stable-x86_64-pc-windows-msvc/lib/rustlib/1stable-x86_64-pc-windows-msvc/lib
  
※ `{USER_NAME}` ... PCのユーザネーム


## 3.実行
 1) `cargo run` で実行
 2) キーボードの十字キーに応じて画面内のオブジェクトが動く
