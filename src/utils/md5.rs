use md5::compute;

#[allow(dead_code)]
pub fn create_md5(dig:String)->String{
  let digest = compute(dig);
  return format!("{:x}",digest)
}