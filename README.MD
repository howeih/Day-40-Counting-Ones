Day 40: Counting Ones
<br>
Given a stream of zeroes and ones, the goal today is to count number of ones in an arbitrarily large window.
<br>
That is surprisingly difficult problem. If there are 10⁹ elements in a day and you are queried for the past 6 months, you can’t just keep all the stream in memory, and still, you have to keep the track somehow.
<br>
What we can do is to estimate the number. Instead of representing a single large window, we only keep buckets representing consecutive windows containing exactly 2^k ones together with timestamps of the last occurrence. To limit resources, buckets of the same size can be grouped together into twice as large bucket as data are streamed.
<br>
When querying number of ones, we take the buckets that fit into the requested window and half of the last bucket that doesn’t fit. The estimate will than make 50% error at worst.
<br>
The algorithm requires O(log²n) memory and a single query takes O(log n) time. What’s the best, we can store zero in O(1) time and we can store one in O(1) amortized time.
<br>
run:
```
fn main() {
    let mut rng = thread_rng();
    let n = 10i32.pow(5);
    let join_handler = {
        let (tx, rx): (Sender<Data>, Receiver<Data>) = mpsc::channel();
        let join_handler = stream_counter(rx);
        for _ in 0..n {
            let x: f64 = rng.gen();
            tx.send(Data::Code(x >= 0.5)).ok();
        }
        let start = 0.99;
        for i in 0..5 {
            let k = n as f64 * (start - i as f64 * 0.99 / 4.);
            tx.send(Data::Estimate(k as i32, n)).ok();
        }
        join_handler
    };
    join_handler.join().ok();
}
```
result:
```
last 1000 bits: 731
last 25750 bits: 17115
last 50500 bits: 33499
last 75250 bits: 49883
last 100000 bits: 49883
```