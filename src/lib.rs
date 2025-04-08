use anyhow::{anyhow, Context, Result};
use info_line::RFLogInfoLine;
use log::Log;

pub mod info_line;
pub mod log;

#[derive(Debug, Clone)]
pub struct RFLogFile {
  pub info_line: RFLogInfoLine,
  pub logs: Vec<Log>
}

pub fn parse(body: String) -> Result<RFLogFile> {
  let mut lines = body.lines();

  let info_line = lines
    .next()
    .context(anyhow!("Invalid format: Unable to get RFLogInfoLine"))?;

  let mut logs = Vec::new();

  for (line, log) in lines.enumerate() {
    logs.push(log::parse(line, log)?);
  }

  Ok(
    RFLogFile {
      info_line: info_line::parse(info_line)?,
      logs
    }
  )
}