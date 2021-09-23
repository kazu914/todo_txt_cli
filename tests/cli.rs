#[cfg(test)]
mod integrate_test {
    use assert_cmd::prelude::*;
    use std::io::Read;
    use std::process::Command;
    use tempfile::NamedTempFile;
    use todo_txt::helper;

    #[test]
    fn add() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        let mut cmd = Command::cargo_bin("todo_txt")?;
        cmd.arg("-f")
            .arg(file.path())
            .arg("add")
            .arg("-p")
            .arg("A")
            .arg("-P")
            .arg("projectA,projectB")
            .arg("-C")
            .arg("contextA,contextB")
            .arg("todo text");
        cmd.assert().success();

        let today = helper::get_today();
        let expected = format!(
            "(A) {} todo text +projectA +projectB @contextA @contextB\n",
            today
        );

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        assert_eq!(buf, expected);
        Ok(())
    }
}
