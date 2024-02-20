use crate::git::repository::Repository;
use git2::{Diff, DiffOptions};

impl Repository {
    pub(crate) fn get_diff(&self, include_untracked: bool) -> Option<Diff> {
        let mut options = DiffOptions::new();
        options.include_untracked(include_untracked);

        let diff = match &self.get_head() {
            Some(head) => self
                .0
                .diff_tree_to_index(head.as_tree(), None, Some(&mut options)),
            None => self
                .0
                .diff_tree_to_workdir_with_index(None, Some(&mut options)),
        };

        match diff {
            Ok(diff) => {
                if diff.deltas().len() > 0 {
                    Some(diff)
                } else {
                    None
                }
            }
            Err(..) => None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::test_helpers::git_init_no_gpg;
    use anyhow::Result;
    use cmd_lib::run_cmd;
    use sealed_test::prelude::*;

    #[sealed_test]
    fn get_diff_some() -> Result<()> {
        let repo = git_init_no_gpg()?;

        // Arrange
        run_cmd!(
            echo changes > file;
            git add .;
        )?;

        // Act
        let diffs = repo.get_diff(false);

        // Assert
        assert!(diffs.is_some());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_none() -> Result<()> {
        let repo = git_init_no_gpg()?;

        // Arrange
        run_cmd!(
            echo changes > file;
        )?;

        // Act
        let diffs = repo.get_diff(false);

        // Assert
        assert!(diffs.is_none());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_include_untracked_some() -> Result<()> {
        let repo = git_init_no_gpg()?;

        // Arrange
        run_cmd!(
            echo changes > file;
        )?;

        // Act
        let diffs = repo.get_diff(true);

        // Assert
        assert!(diffs.is_some());
        Ok(())
    }

    #[sealed_test]
    fn get_diff_include_untracked_none() -> Result<()> {
        // Arrange
        let repo = git_init_no_gpg()?;

        // Act
        let diffs = repo.get_diff(true);

        // Assert
        assert!(diffs.is_none());
        Ok(())
    }
}
