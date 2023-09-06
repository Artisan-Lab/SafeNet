pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    // let args = args
    //     .collect_str(InvalidEncodingHandling::ConvertLossy)
    //     .accept_any();

    // let matches = uu_app().get_matches_from(args);

    // let silent = matches.contains_id(options::SILENT);

    // Call libc function ttyname
    let tty = unsafe {
        let ptr = libc::ttyname(libc::STDIN_FILENO);
        if !ptr.is_null() {
            String::from_utf8_lossy(CStr::from_ptr(ptr).to_bytes()).to_string()
        } else {
            "".to_owned()
        }
    };

    // let mut stdout = std::io::stdout();

    // if !silent {
    //     let write_result = if !tty.chars().all(|c| c.is_whitespace()) {
    //         writeln!(stdout, "{}", tty)
    //     } else {
    //         writeln!(stdout, "not a tty")
    //     };
    //     if write_result.is_err() || stdout.flush().is_err() {
    //         // Don't return to prevent a panic later when another flush is attempted
    //         // because the `uucore_procs::main` macro inserts a flush after execution for every utility.
    //         std::process::exit(3);
    //     }
    // }

    // if atty::is(atty::Stream::Stdin) {
    //     Ok(())
    // } else {
    //     Err(libc::EXIT_FAILURE.into())
    // }
}