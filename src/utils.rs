use crate::Error;

pub fn env_var<T: std::str::FromStr>(name: &str) -> Result<T, Error>
where
    T::Err: std::fmt::Display,
{
    Ok(std::env::var(name)
        .map_err(|_| format!("Missing {name}"))?
        .parse()
        .map_err(|e| format!("Invalid {name}: {e}"))?)
}
