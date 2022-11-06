mod use_ex;
mod c;
mod link_list;
use use_ex::use_fn;
use c::c_fn;
use link_list::link_fn;
#[allow(dead_code)]
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect (event: WebEvent) {
	match event {
		WebEvent::PageLoad => println!("page loaded"),
		WebEvent::PageUnload => println!("page unloaded"),
		WebEvent::KeyPress(c) => println!("press '{}'.", c),
		WebEvent::Paste(s) => println!("pasted \"{}\".", s),
		WebEvent::Click { x, y } => {
			println!("clicked at x={}, y={}", x, y);
		},
	}
}

pub fn enum_fn() {
  use WebEvent::*;
	let pressed = KeyPress('x');
	let pasted = Paste("my text".to_owned());
	let click = Click { x: 20, y: 80 };
	let load = PageLoad;
	let unload = PageUnload;

	inspect(pressed);
	inspect(pasted);
	inspect(click);
	inspect(load);
	inspect(unload);

  use_fn();
  c_fn();
  link_fn();
}