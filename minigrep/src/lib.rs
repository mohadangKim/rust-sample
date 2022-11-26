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
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  //수명 어노테이션 사용하여 인수의 수명이 리턴값의 수명과 연결 되었음을 컴파일러에 알림
  //contents 인수는 검색 대상이 되는 모든 텍스트를 저장하고 있으며
  //리턴할 값은 그 일부이므로 contents 인수의 수명이 리턴값의 수명과 연결되어야 한다.
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();//query : &str -> String
  let mut results = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) { // contains 는 문자열 슬라이스를 받는다.
      results.push(line);
    }
  }
  results
}
#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
      vec!["safe, fast, productive."],
      search(query, contents)
    );
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }  
}