use std::path;

pub mod base64;
pub mod csv;
pub mod genpass;
pub mod opts;

/// 自定义参数校验器: 校验文件路径是否存在
pub fn validate_path(path: &str) -> Result<String, String> {
    if path == "-" || path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("{} 文件不存在", path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_path() {
        assert_eq!(validate_path("Cargo.toml").unwrap(), "Cargo.toml");
        assert_eq!(validate_path("-").unwrap(), "-");
        assert!(validate_path("not_exist.txt").is_err());
    }
}
