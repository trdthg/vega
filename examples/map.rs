use vega::*;

fn main() -> Result<()> {
    let sc = Context::new()?;
    let vec = vec![1, 2, 3, 4, 5];
    let r = sc.make_rdd(vec, 10);
    let res = r.map(Fn!(|x| x + 1)).collect();
    println!("result: {:?}", res);
    Ok(())
}
