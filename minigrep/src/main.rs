use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
  let args: Vec<String> = env::args().collect();
  let config = Config::new(&args).unwrap_or_else(|err| {
    // 반환이 Ok이면 unwrap과 같이 동작함, 즉 Ok 값을 unwrap 하여 결과값을 반환
    // 반환이 Err이면 클로저를 이용해 unwrap_or_else에 전달한 익명 함수를 호출
    println!("인수를 구문분석하는 동안 오류가 발생했습니다.: {}", err);
    process::exit(1);
  });
  
  println!("검색어 : {}", config.query);
  println!("대상 파일 : {}", config.filename);
  if let Err(e) = run(config) {
    println!("애플리케이션 에러: {}", e);
    process::exit(1);
  }
}

struct Config {
  query: String,
  filename: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &'static str> {
    if args.len() < 3 {
      return Err("필요한 인수가 지정되지 않았습니다.");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
  
    Ok(Config { query, filename })
  }  
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // 성공하였을 경우 함수 기본 반환 값인 ()를 반환
  // 실패하였을 경우 Box<dyn Error> 트레이트 반환
  let contents = fs::read_to_string(config.filename)?;// ? : 에러 발생하였을 경우 호출자에게 에러 반환
  println!("파일 내용:\n{}", contents);

  Ok(())
}