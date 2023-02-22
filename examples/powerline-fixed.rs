use powerline::{modules::*, theme::SimpleTheme};

fn main() -> powerline::R<()> {
	let mut prompt = powerline::Powerline::new();

	prompt.add_module(NewLine::<SimpleTheme>::new());
	prompt.add_module(Fish::<SimpleTheme>::new());
	prompt.add_module(ExecTime::<SimpleTheme>::new());
	prompt.add_module(ExitCode::<SimpleTheme>::new());
	prompt.add_module(Time::<SimpleTheme>::new());
	prompt.add_module(Cwd::<SimpleTheme>::new(25, 2, false));
	prompt.add_module(Git::<SimpleTheme>::new());
	prompt.add_module(ReadOnly::<SimpleTheme>::new());
	prompt.add_module(NewLine::<SimpleTheme>::new());
	prompt.add_module(Cmd::<SimpleTheme>::new());

	println!("{}", prompt);
	Ok(())
}
