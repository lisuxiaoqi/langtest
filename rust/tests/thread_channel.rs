use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;


/*
    channel的节本用法， 生产者，消费者
*/
#[test]
fn test_channel() {
    let (sender, receiver) = channel();

    let s1 = sender.clone();
    let s2 = sender.clone();
    //这里要把原始的sender drop掉，不然receiver会以为还有sender，继续等待
    drop(sender);
    thread::scope(|scope| {
        //最好把自己的sender move到线程中去
        //如果加了手动drop也可以不move，但move是更安全的做法
        scope.spawn(move || {
            for i in 0..3 {
                println!("--S1 send:{}", i);
                s1.send(i);
                thread::sleep(Duration::from_secs(1));
            }
            println!("S1 quit");
            //执行完把sender drop掉，不然receiver会以为还有sender，继续等待
            drop(s1);
        });

        scope.spawn(move || {
            for i in 4..7 {
                println!("--S2 send:{}", i);
                s2.send(i);
                thread::sleep(Duration::from_secs(1));
            }
            println!("S2 quit");
            //执行完把sender drop掉，不然receiver会以为还有sender，继续等待
            drop(s2);
        });

        scope.spawn(move || {
            while let Ok(v) = receiver.recv() {
                println!("recv: {}", v);
            }
            println!("Receiver quit");
        });
    })
}