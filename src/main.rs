use Basic::Queue;
fn main(){
    let mut x = Queue::new(Vec::new());
    x.append(456);
    x.append(60);
    x.append(123);
    x.append(12314);
    x.append(456);
    println!("{}", x);
    x.popfront();
    x.popfront();
    x.popfront();
    x.popfront();
    x.popfront();
    println!("{}", x);
}