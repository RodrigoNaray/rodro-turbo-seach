use std::ffi::OsString;

pub const USAGE: &str = "Uso: rodro-turbo-search [toggle|quit|--help]\n\nSin argumentos: iniciar residente y oculto.\ntoggle: mostrar u ocultar (inicia el residente si es necesario).\nquit: finalizar la instancia residente.";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Start,
    Toggle,
    Quit,
    Help,
}

impl Command {
    pub fn parse(args: impl IntoIterator<Item = OsString>) -> Result<Self, &'static str> {
        let mut args = args.into_iter();
        let command = match args.next() {
            None => Self::Start,
            Some(arg) => match arg.to_str() {
                Some("toggle") => Self::Toggle,
                Some("quit") => Self::Quit,
                Some("--help" | "-h") => Self::Help,
                _ => return Err("Comando desconocido."),
            },
        };
        if args.next().is_some() {
            return Err("Se admite un único comando.");
        }
        Ok(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_supported_commands() {
        for (args, expected) in [
            (vec![], Command::Start),
            (vec!["toggle"], Command::Toggle),
            (vec!["quit"], Command::Quit),
            (vec!["--help"], Command::Help),
            (vec!["-h"], Command::Help),
        ] {
            assert_eq!(
                Command::parse(args.into_iter().map(OsString::from)),
                Ok(expected)
            );
        }
    }

    #[test]
    fn rejects_unknown_or_extra_arguments() {
        for args in [
            vec!["unknown"],
            vec!["toggle", "quit"],
            vec!["--help", "extra"],
        ] {
            assert!(Command::parse(args.into_iter().map(OsString::from)).is_err());
        }
    }

    #[cfg(unix)]
    #[test]
    fn rejects_non_unicode_arguments() {
        use std::os::unix::ffi::OsStringExt;
        assert!(Command::parse([OsString::from_vec(vec![0xff])]).is_err());
    }
}
