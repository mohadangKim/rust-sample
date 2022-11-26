use std::error::Error;
use std::fs;


pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("필요한 인수가 지정되지 않았습니다.");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
  
    Ok(Config { query, filename })
  }  
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // 성공하였을 경우 함수 기본 반환 값인 ()를 반환
  // 실패하였을 경우 Box<dyn Error> 트레이트 반환
  let contents = fs::read_to_string(config.filename)?;// ? : 에러 발생하였을 경우 호출자에게 에러 반환
  println!("파일 내용:\n{}", contents);

  Ok(())
}