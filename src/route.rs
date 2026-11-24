use anyhow::{bail, Result};

pub fn validate(pattern: &str) -> Result<()> {
    if pattern.contains(' ') {
        bail!("route pattern must not contain spaces: {:?}", pattern);
    }
    if pattern.contains('?') {
        bail!("route pattern must not contain query parameters (?): {:?}", pattern);
    }
    if pattern.contains('#') {
        bail!("route pattern must not contain fragments (#): {:?}", pattern);
    }

    // Strip optional scheme
    let without_scheme = if let Some(rest) = pattern.strip_prefix("https://") {
        rest
    } else if let Some(rest) = pattern.strip_prefix("http://") {
        rest
    } else {
        pattern
    };

    // Split hostname from path
    let (hostname, path) = match without_scheme.find('/') {
        Some(i) => (&without_scheme[..i], Some(&without_scheme[i + 1..])),
        None => (without_scheme, None),
    };

    // Hostname must contain a dot (after stripping a leading wildcard)
    let hostname_core = hostname
        .strip_prefix("*.")
        .or_else(|| hostname.strip_prefix('*'))
        .unwrap_or(hostname);

    if !hostname_core.contains('.') {
        bail!(
            "route pattern hostname must contain a dot: {:?}",
            pattern
        );
    }

    // No wildcard in the middle of the hostname
    if hostname_core.contains('*') {
        bail!(
            "route pattern wildcard in hostname must only appear at the start: {:?}",
            pattern
        );
    }

    // In the path, wildcard may only appear at the end
    if let Some(p) = path {
        let star_pos = p.find('*');
        if let Some(pos) = star_pos {
            if pos != p.len() - 1 {
                bail!(
                    "route pattern wildcard in path must only appear at the end: {:?}",
                    pattern
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_patterns() {
        let valid = [
            "example.com/*",
            "example.com/",
            "example.com",
            "example.com/path/*",
            "*.example.com/*",
            "*example.com/*",
            "https://example.com/*",
            "http://example.com/path/*",
        ];
        for p in valid {
            assert!(validate(p).is_ok(), "expected valid: {p}");
        }
    }

    #[test]
    fn rejects_spaces() {
        assert!(validate("exam ple.com/*").is_err());
    }

    #[test]
    fn rejects_query_string() {
        assert!(validate("example.com/?foo=bar").is_err());
    }

    #[test]
    fn rejects_fragment() {
        assert!(validate("example.com/#section").is_err());
    }

    #[test]
    fn rejects_no_dot_in_hostname() {
        assert!(validate("localhost/*").is_err());
        assert!(validate("example/*").is_err());
    }

    #[test]
    fn rejects_infix_wildcard_in_hostname() {
        assert!(validate("exam*.com/*").is_err());
    }

    #[test]
    fn rejects_infix_wildcard_in_path() {
        assert!(validate("example.com/*.jpg").is_err());
        assert!(validate("example.com/*/foo").is_err());
    }
}
