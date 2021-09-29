#[cfg(test)]
mod integrate_test {
    use assert_cmd::prelude::*;
    use std::io::Read;
    use std::process::Command;
    use tempfile::NamedTempFile;
    use test_case::test_case;
    use todo_txt::helper;

    #[test]
    fn add() -> Result<(), Box<dyn std::error::Error>> {
        let mut file = NamedTempFile::new()?;
        let res = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("add")
            .arg("-p")
            .arg("A")
            .arg("-P")
            .arg("projectA,projectB")
            .arg("-C")
            .arg("contextA,contextB")
            .arg("todo text")
            .assert();
        res.success();

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

    #[test_case("projectA projectB", "contextA")]
    #[test_case("projectA", "contextA contextB")]
    #[test_case("projectA projectB", "contextA contextB")]
    fn failed_to_add(projects: &str, contexts: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = NamedTempFile::new()?;
        let res = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("add")
            .arg("-P")
            .arg(projects)
            .arg("-C")
            .arg(contexts)
            .arg("todo text")
            .assert();
        res.failure();
        Ok(())
    }

    #[test]
    fn done() -> Result<(), Box<dyn std::error::Error>> {
        let (_, path) = NamedTempFile::new()?.keep()?;
        let _ = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(&path)
            .arg("add")
            .arg("-p")
            .arg("A")
            .arg("-P")
            .arg("project")
            .arg("-C")
            .arg("context")
            .arg("todo text")
            .ok();

        let res = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(&path)
            .arg("done")
            .arg("0")
            .assert();
        res.success();

        let today = helper::get_today();
        let expected = format!("x (A) {} {} todo text +project @context\n", today, today);

        let buf = std::fs::read_to_string(&path)?;
        assert_eq!(buf, expected);
        std::fs::remove_file(&path)?;
        Ok(())
    }

    #[test]
    fn list() -> Result<(), Box<dyn std::error::Error>> {
        let file = NamedTempFile::new()?;
        let _ = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("add")
            .arg("-p")
            .arg("A")
            .arg("-P")
            .arg("project")
            .arg("-C")
            .arg("context")
            .arg("todo text")
            .ok();

        let _ = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("add")
            .arg("-p")
            .arg("B")
            .arg("-C")
            .arg("contextB")
            .arg("second todo")
            .ok();

        let _ = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("done")
            .arg("1")
            .ok();

        let res = Command::cargo_bin("todo_txt")?
            .arg("-f")
            .arg(file.path())
            .arg("list")
            .assert();

        let today = helper::get_today();
        let expected = format!(
            "0: (A) {} todo text +project @context\n1: x (B) {} {} second todo @contextB\n",
            today, today, today
        );
        res.stdout(expected).success();

        Ok(())
    }
}
