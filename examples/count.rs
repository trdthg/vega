use vega::*;

fn main() -> Result<()> {
  let sc = Context::new()?;
  let vec = vec![1, 2, 3, 4, 5];
  let r = sc.make_rdd(vec, 10);
  let f = Fn!(|x: i32| -> i32 { x + 1 });
  let res: i32 = f(1);
  println!("result: {:?}", res);
  let res = r.map(Fn!(|x| x + 1)).collect();
  println!("result: {:?}", res);
  Ok(())
}
