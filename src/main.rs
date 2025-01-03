mod events;
mod trace;

use miette::IntoDiagnostic;

fn run() -> miette::Result<u8> {
    let mut shell = prompt::Mishell::new().into_diagnostic()?;
    shell.run().into_diagnostic()?;

    let exit_status = shell.engine().as_ref().last_exit_status();
    Ok(exit_status)
}

fn main() -> miette::Result<()> {
    trace::init();

    let exit_code = match run() {
        Ok(code) => code as i32,
        Err(e) => {
            tracing::error!("error: {:#}", e);
            1
        }
    };

    std::process::exit(exit_code);

    // let temp_file = temp_dir().join("temp_file.nu");

    // let mut history_session_id = Reedline::create_history_session_id();
    // let history = Box::new(
    //     reedline::SqliteBackedHistory::with_file(
    //         "history.sqlite3".into(),
    //         history_session_id,
    //         Some(chrono::Utc::now()),
    //     )
    //     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    //     .into_diagnostic()?,
    // );

    // let mut line_editor = Reedline::create()
    //     .with_history_session_id(history_session_id)
    //     .with_history(history)
    //     .with_history_exclusion_prefix(Some(" ".to_string()))
    //     .with_quick_completions(true)
    //     .with_partial_completions(true)
    //     .with_cursor_config(CursorConfig {
    //         vi_insert: Some(SetCursorStyle::BlinkingBar),
    //         vi_normal: Some(SetCursorStyle::SteadyBlock),
    //         emacs: None,
    //     })
    //     .use_bracketed_paste(true)
    //     .use_kitty_keyboard_enhancement(true)
    //     .with_hinter(Box::new(
    //         DefaultHinter::default().with_style(Style::new().fg(Color::DarkGray)),
    //     ))
    //     .with_validator(Box::new(DefaultValidator))
    //     .with_ansi_colors(true)
    //     .with_menu(ReedlineMenu::EngineCompleter(Box::new(
    //         ColumnarMenu::default().with_name("completion_menu"),
    //     )))
    //     .with_menu(ReedlineMenu::HistoryMenu(Box::new(
    //         ListMenu::default().with_name("history_menu"),
    //     )))
    //     .with_edit_mode({
    //         let mut normal_keybindings = default_vi_normal_keybindings();
    //         let mut insert_keybindings = default_vi_insert_keybindings();

    //         add_menu_keybindings(&mut normal_keybindings);
    //         add_menu_keybindings(&mut insert_keybindings);

    //         add_newline_keybinding(&mut insert_keybindings);

    //         Box::new(Vi::new(insert_keybindings, normal_keybindings))
    //     })
    //     .with_buffer_editor(
    //         {
    //             let mut command = Command::new("vi");
    //             command.arg(&temp_file);
    //             command
    //         },
    //         temp_file,
    //     );

    // let prompt = DefaultPrompt::default();

    // loop {
    //     match line_editor.read_line(&prompt) {
    //         Ok(Signal::CtrlD) => {
    //             break;
    //         }
    //         Ok(Signal::Success(buffer)) => {
    //             let start = std::time::Instant::now();
    //             if !buffer.is_empty() {
    //                 line_editor
    //                     .update_last_command_context(&|mut c: reedline::HistoryItem| {
    //                         c.start_timestamp = Some(chrono::Utc::now());
    //                         c.hostname =
    //                             Some(gethostname::gethostname().to_string_lossy().to_string());
    //                         c.cwd = std::env::current_dir()
    //                             .ok()
    //                             .map(|e| e.to_string_lossy().to_string());
    //                         c
    //                     })
    //                     .expect("todo: error handling");
    //             }

    //             if (buffer.trim() == "exit") || (buffer.trim() == "logout") {
    //                 break;
    //             }
    //             if buffer.trim() == "clear" {
    //                 line_editor.clear_scrollback().into_diagnostic()?;
    //                 continue;
    //             }
    //             // Get the full history
    //             if buffer.trim() == "history" {
    //                 line_editor.print_history().into_diagnostic()?;
    //                 continue;
    //             }
    //             // Get the history only pertinent to the current session
    //             if buffer.trim() == "history session" {
    //                 line_editor.print_history_session().into_diagnostic()?;
    //                 continue;
    //             }
    //             // Get this history session identifier
    //             if buffer.trim() == "history sessionid" {
    //                 line_editor.print_history_session_id().into_diagnostic()?;
    //                 continue;
    //             }
    //             // Toggle between the full history and the history pertinent to the current session
    //             if buffer.trim() == "toggle history_session" {
    //                 let hist_session_id = if history_session_id.is_none() {
    //                     // If we never created a history session ID, create one now
    //                     let sesh = Reedline::create_history_session_id();
    //                     history_session_id = sesh;
    //                     sesh
    //                 } else {
    //                     history_session_id
    //                 };
    //                 line_editor
    //                     .toggle_history_session_matching(hist_session_id)
    //                     .into_diagnostic()?;
    //                 continue;
    //             }
    //             if buffer.trim() == "clear-history" {
    //                 let hstry = Box::new(line_editor.history_mut());
    //                 hstry
    //                     .clear()
    //                     .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    //                     .into_diagnostic()?;
    //                 continue;
    //             }
    //             println!("Our buffer: {buffer}");

    //             if !buffer.is_empty() {
    //                 line_editor
    //                     .update_last_command_context(&|mut c| {
    //                         c.duration = Some(start.elapsed());
    //                         c.exit_status = Some(0);
    //                         c
    //                     })
    //                     .expect("todo: error handling");
    //             }

    //             let _tokens = tokenizer(&buffer);

    //             debug_tokens(&_tokens);

    //             println!("---Debug log ended");

    //             let mut parser = Parser::new(_tokens);

    //             let ast = parser.parse().map_err(|err| miette!(err))?;

    //             println!("{:#?}", ast);
    //         }
    //         Ok(Signal::CtrlC) => {
    //             // Prompt has been cleared and should start on the next line
    //         }
    //         Err(err) => {
    //             println!("Error: {err:?}");
    //         }
    //     }

    // parse input
    // let mut input = String::new();

    // io::stdin().read_line(&mut input).into_diagnostic()?;

    // // tokenize input
    // let _tokens = tokenizer(&input);

    // debug_tokens(&_tokens);

    // println!("---Debug log ended");

    // let mut parser = Parser::new(_tokens);

    // let ast = parser.parse().expect("Failed to parse nigger");

    // println!("{:#?}", ast);
    // parse input

    // execute read expression through the engine?
    // let mut parts = input.split_whitespace();
    // let command = parts.next().unwrap();
    // let args = parts;

    // let mut child = std::process::Command::new(command)
    //     .args(args)
    //     .spawn()
    //     .into_diagnostic()?;
    // child.wait().into_diagnostic()?;
    // }
}
