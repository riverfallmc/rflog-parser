use anyhow::{bail, Context, Result};
use regex::Regex;

const LOG_MARK: &str = "Riverfall Launcher Log Format";

macro_rules! get_element {
    ($i:ident,$n:tt,$ii:tt,$t:ty) => {
      $i.get($ii)
        .context(anyhow::anyhow!("Unable to get element {} (№{}) of RFLogMark", $n, $ii))?
        .as_str()
        .parse::<$t>().ok().context(anyhow::anyhow!("Unable to cast element {} (№{}, &str) to type {}", $n, $ii, stringify!($t))) // idk will it gonna work or not
    };
}

#[derive(Debug, Clone)]
pub struct Version {
  pub major: u16,
  pub minor: u16,
  pub patch: u16
}

impl ToString for Version {
  fn to_string(&self) -> String {
    format!("{}.{}.{}", self.major, self.minor, self.patch)
  }
}

#[derive(Debug, Clone)]
pub struct RFLogInfoLine {
  pub launcher_version: Version,
  pub player_nick: String,
  pub game_client: String,
  pub os: String,
  pub os_version: String
}

pub(crate) fn parse(line: &str) -> Result<RFLogInfoLine> {
  let re = Regex::new(r"([^:]+):\[(\d+)\.(\d+)\.(\d+);([^;]+);([^;]+);([^;]+);([^\]]+)\]").unwrap();

  if let Some(caps) = re.captures(line) {
    let log_mark = caps.get(1).context("Invalid format: Unable to get RFLogMark")?.as_str();

    if log_mark != LOG_MARK {
      bail!("Invalid format: Unable to get RFLogMark");
    }

    let major = get_element!(caps, "Version (major)", 2, u16)?;
    let minor = get_element!(caps, "Version (minor)", 3, u16)?;
    let patch = get_element!(caps, "Version (patch)", 4, u16)?;
    let player_nick = get_element!(caps, "Username", 5, String)?;
    let os = get_element!(caps, "OS", 6, String)?;
    let os_version = get_element!(caps, "OS Version", 7, String)?;
    let game_client = get_element!(caps, "Game Client", 8, String)?;

    Ok(
      RFLogInfoLine {
        launcher_version: Version { major, minor, patch },
        player_nick,
        game_client,
        os,
        os_version
      }
    )
  } else {
    bail!("Invalid format: Unable to get RFLogMark")
  }
}