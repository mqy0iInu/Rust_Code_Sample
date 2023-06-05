// std::thread: Rustの標準ライブラリに含まれるモジュール。スレッドの作成と制御を提供する
//              httpsstd::thread::spawn関数を使用して新しいスレッドを生成し、
//              std::thread::JoinHandleを介してスレッドの実行結果を受け取ることができる。

// std::sync::Mutex: 複数のスレッド間でデータの安全な共有と同期を行うためのロック機構。
//                   std::sync::MutexGuardを使用してロックされたデータへのアクセスを管理する。

// std::sync::Arc: 複数のスレッド間で所有権を共有するためのスマートポインタ。
//                 Arcは参照カウントを使用して複数のスレッドでの所有権を追跡し、データの安全な共有を可能する。

// std::sync::RwLock: 読み込み操作と書き込み操作を同時に行うことができる読み取り優先のロック機構。
//                    複数のスレッドが同時に読み取りアクセスを行えるが、書き込み操作が行われている場合は他のスレッドのアクセスをブロックする。

// std::sync::Barrier: 複数のスレッドが指定された箇所で一時停止し、全てのスレッドが到達するまで待機するための同期機構。
//                     全てのスレッドが到達すると、一斉に再開されます。

// std::sync::Condvar: 条件変数を使用してスレッド間の通信と同期を行う。
//                     スレッドが特定の条件を待機し、他のスレッドが条件を満たした時に通知することができる。

// std::sync::Once: 特定の処理がスレッドごとに一度だけ実行されることを保証する。
//                  複数のスレッドが同時に処理を実行しても、その処理は一度だけ実行される。

// std::sync::Atomic: アトミックな操作を提供する型。
//                    複数のスレッドが同時に値を読み書きする場合にデータ競合を避けるために使用される。

// ★RustのスレッドにはFreeRTOSの優先度がないので
// ★スケジューリングは実行環境によって異なることに注意！

use std::thread;
use std::time::Duration;

fn main()
{
    // スレッド1
    let thread1 = thread::spawn(|| {
        for _ in 0..10 {
            println!("スレッド1");
            thread::sleep(Duration::from_millis(100));
        }
    });

    // スレッド2
    let thread2 = thread::spawn(|| {
        for _ in 0..10 {
            println!("スレッド2");
            thread::sleep(Duration::from_millis(100));
        }
    });

    // スレッド3
    let thread3 = thread::spawn(|| {
        for _ in 0..10 {
            println!("スレッド3");
            thread::sleep(Duration::from_millis(100));
        }
    });

    // スレッドの終了を待つ
    thread1.join().expect("スレッド1がクラッシュしました");
    thread2.join().expect("スレッド2がクラッシュしました");
    thread3.join().expect("スレッド3がクラッシュしました");
}
