use anyhow::Result;
use chrono::NaiveTime;

#[derive(Debug, Clone)]
pub enum LogKind {
  Out,
  Err
}

#[derive(Debug, Clone)]
pub struct Log {
  pub log_line: usize,
  pub kind: LogKind,
  pub time: Option<NaiveTime>,
  pub thread: String,
  pub executor: String,
  pub data: Vec<String>
}

pub fn parse(line: usize, log: &str) -> Result<Log> {
  use regex::Regex;

  let log = log.trim();
  if log.is_empty() {
    anyhow::bail!("Пустая строка лога");
  }

  let kind = if log.starts_with("[OUT]") {
    LogKind::Out
  } else if log.starts_with("[ERR]") {
    LogKind::Err
  } else {
    anyhow::bail!("Неизвестный тип лога");
  };

  // Убираем [OUT] или [ERR] и тримим
  let content = log
    .strip_prefix("[OUT]")
    .or_else(|| log.strip_prefix("[ERR]"))
    .unwrap()
    .trim();

  // Если пусто — типа просто "[OUT]" — возвращаем пустой лог
  if content.is_empty() {
    return Ok(Log {
      log_line: line,
      kind,
      time: None,
      thread: String::new(),
      executor: String::new(),
      data: vec![],
    });
  }

  let re_full = Regex::new(r"^\[(\d{2}:\d{2}:\d{2})] \[([^\]]+)] \[([^\]]+)]\:?\s?(.*)$")?;
  let re_simple = Regex::new(r"^\[([^\]]+)] \[([^\]]+)] (.+)$")?;

  if let Some(caps) = re_full.captures(content) {
    let time = Some(NaiveTime::parse_from_str(caps.get(1).unwrap().as_str(), "%H:%M:%S")?);
    let thread = caps.get(2).unwrap().as_str().to_string();
    let executor = caps.get(3).unwrap().as_str().to_string();
    let data_str = caps.get(4).map(|m| m.as_str().trim()).unwrap_or("").to_string();
    let data = if data_str.is_empty() { vec![] } else { vec![data_str] };

    Ok(Log {
      log_line: line,
      kind,
      time,
      thread,
      executor,
      data,
    })
  } else if let Some(caps) = re_simple.captures(content) {
    let thread = caps.get(1).unwrap().as_str().to_string();
    let executor = caps.get(2).unwrap().as_str().to_string();
    let data_str = caps.get(3).unwrap().as_str().to_string();

    Ok(Log {
      log_line: line,
      kind,
      time: None,
      thread,
      executor,
      data: vec![data_str],
    })
  } else {
    // просто continuation-строка
    Ok(Log {
      log_line: line,
      kind,
      time: None,
      thread: String::new(),
      executor: String::new(),
      data: vec![content.to_string()],
    })
  }
}
