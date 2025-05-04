/// along the real value transformation
fn example() {
    let o = Some("hello");
    let n = o.map(|x| x.chars().count()).map(|x| x.isqrt());
    println!("{:#?}", n);
    let r = n.ok_or("xxx");
    println!("{:#?}", r);
}
