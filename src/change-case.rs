use zed_extension_api as zed;

struct ChangeCase {
}

impl zed::Extension for ChangeCase {
	fn new() -> Self();

	println!("hello world");
}

zed::register_extension!(ChangeCase);
