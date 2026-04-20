//! Bundled skills — compile-time embedded SKILL.md files.
//!
//! Ships 60 prompt-only skills inside the OpenFang binary via `include_str!()`.
//! User-installed skills with the same name override bundled ones.

use crate::openclaw_compat::convert_skillmd_str;
use crate::SkillManifest;

/// Return all bundled (name, raw SKILL.md content) pairs.
pub fn bundled_skills() -> Vec<(&'static str, &'static str)> {
    vec![]
}

/// Parse a bundled SKILL.md into a `SkillManifest`.
pub fn parse_bundled(name: &str, content: &str) -> Result<SkillManifest, crate::SkillError> {
    let converted = convert_skillmd_str(name, content)?;
    Ok(converted.manifest)
}

/// Parse a bundled SKILL.md into its full converted form (manifest + declared
/// config vars). Used by the registry loader so it can resolve/inject config.
pub fn parse_bundled_full(
    name: &str,
    content: &str,
) -> Result<crate::openclaw_compat::ConvertedSkillMd, crate::SkillError> {
    convert_skillmd_str(name, content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_bundled_skills_parse() {
        let skills = bundled_skills();
        for (name, content) in &skills {
            let result = parse_bundled(name, content);
            assert!(
                result.is_ok(),
                "Failed to parse bundled skill '{}': {:?}",
                name,
                result.err()
            );
            let manifest = result.unwrap();
            assert!(
                !manifest.skill.name.is_empty(),
                "Bundled skill '{}' has empty name",
                name
            );
            assert!(
                !manifest.skill.description.is_empty(),
                "Bundled skill '{}' has empty description",
                name
            );
            assert!(
                manifest.prompt_context.is_some(),
                "Bundled skill '{}' has no prompt context",
                name
            );
            assert_eq!(
                manifest.source,
                Some(crate::SkillSource::Bundled),
                "Bundled skill '{}' should have Bundled source",
                name
            );
        }
    }

    #[test]
    fn test_bundled_skills_pass_security_scan() {
        use crate::verify::SkillVerifier;

        let skills = bundled_skills();
        for (name, content) in &skills {
            let manifest = parse_bundled(name, content).unwrap();
            if let Some(ref ctx) = manifest.prompt_context {
                let warnings = SkillVerifier::scan_prompt_content(ctx);
                let has_critical = warnings
                    .iter()
                    .any(|w| matches!(w.severity, crate::verify::WarningSeverity::Critical));
                assert!(
                    !has_critical,
                    "Bundled skill '{}' has critical security warnings: {:?}",
                    name, warnings
                );
            }
        }
    }
}
