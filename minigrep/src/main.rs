use std::env;
use std::process;

use minigrep::Config;

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
  if let Err(e) = minigrep::run(config) {
    println!("애플리케이션 에러: {}", e);
    process::exit(1);
  }
}