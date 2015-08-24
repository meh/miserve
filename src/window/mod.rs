mod proxy;
pub use self::proxy::Proxy;

mod window;
pub use self::window::Window;

use msg::constellation_msg::Key;
use glutin::VirtualKeyCode;

pub fn key(key: VirtualKeyCode) -> Result<Key, ()> {
	match key {
		VirtualKeyCode::A => Ok(Key::A),
		VirtualKeyCode::B => Ok(Key::B),
		VirtualKeyCode::C => Ok(Key::C),
		VirtualKeyCode::D => Ok(Key::D),
		VirtualKeyCode::E => Ok(Key::E),
		VirtualKeyCode::F => Ok(Key::F),
		VirtualKeyCode::G => Ok(Key::G),
		VirtualKeyCode::H => Ok(Key::H),
		VirtualKeyCode::I => Ok(Key::I),
		VirtualKeyCode::J => Ok(Key::J),
		VirtualKeyCode::K => Ok(Key::K),
		VirtualKeyCode::L => Ok(Key::L),
		VirtualKeyCode::M => Ok(Key::M),
		VirtualKeyCode::N => Ok(Key::N),
		VirtualKeyCode::O => Ok(Key::O),
		VirtualKeyCode::P => Ok(Key::P),
		VirtualKeyCode::Q => Ok(Key::Q),
		VirtualKeyCode::R => Ok(Key::R),
		VirtualKeyCode::S => Ok(Key::S),
		VirtualKeyCode::T => Ok(Key::T),
		VirtualKeyCode::U => Ok(Key::U),
		VirtualKeyCode::V => Ok(Key::V),
		VirtualKeyCode::W => Ok(Key::W),
		VirtualKeyCode::X => Ok(Key::X),
		VirtualKeyCode::Y => Ok(Key::Y),
		VirtualKeyCode::Z => Ok(Key::Z),

		VirtualKeyCode::Numpad0 => Ok(Key::Kp0),
		VirtualKeyCode::Numpad1 => Ok(Key::Kp1),
		VirtualKeyCode::Numpad2 => Ok(Key::Kp2),
		VirtualKeyCode::Numpad3 => Ok(Key::Kp3),
		VirtualKeyCode::Numpad4 => Ok(Key::Kp4),
		VirtualKeyCode::Numpad5 => Ok(Key::Kp5),
		VirtualKeyCode::Numpad6 => Ok(Key::Kp6),
		VirtualKeyCode::Numpad7 => Ok(Key::Kp7),
		VirtualKeyCode::Numpad8 => Ok(Key::Kp8),
		VirtualKeyCode::Numpad9 => Ok(Key::Kp9),

		VirtualKeyCode::Key0 => Ok(Key::Num0),
		VirtualKeyCode::Key1 => Ok(Key::Num1),
		VirtualKeyCode::Key2 => Ok(Key::Num2),
		VirtualKeyCode::Key3 => Ok(Key::Num3),
		VirtualKeyCode::Key4 => Ok(Key::Num4),
		VirtualKeyCode::Key5 => Ok(Key::Num5),
		VirtualKeyCode::Key6 => Ok(Key::Num6),
		VirtualKeyCode::Key7 => Ok(Key::Num7),
		VirtualKeyCode::Key8 => Ok(Key::Num8),
		VirtualKeyCode::Key9 => Ok(Key::Num9),

		VirtualKeyCode::Return   => Ok(Key::Enter),
		VirtualKeyCode::Space    => Ok(Key::Space),
		VirtualKeyCode::Escape   => Ok(Key::Escape),
		VirtualKeyCode::Equals   => Ok(Key::Equal),
		VirtualKeyCode::Minus    => Ok(Key::Minus),
		VirtualKeyCode::Back     => Ok(Key::Backspace),
		VirtualKeyCode::PageDown => Ok(Key::PageDown),
		VirtualKeyCode::PageUp   => Ok(Key::PageUp),

		VirtualKeyCode::Insert => Ok(Key::Insert),
		VirtualKeyCode::Home   => Ok(Key::Home),
		VirtualKeyCode::Delete => Ok(Key::Delete),
		VirtualKeyCode::End    => Ok(Key::End),

		VirtualKeyCode::Left  => Ok(Key::Left),
		VirtualKeyCode::Up    => Ok(Key::Up),
		VirtualKeyCode::Right => Ok(Key::Right),
		VirtualKeyCode::Down  => Ok(Key::Down),

		VirtualKeyCode::Apostrophe => Ok(Key::Apostrophe),
		VirtualKeyCode::Backslash  => Ok(Key::Backslash),
		VirtualKeyCode::Comma      => Ok(Key::Comma),
		VirtualKeyCode::Grave      => Ok(Key::GraveAccent),
		VirtualKeyCode::LBracket   => Ok(Key::LeftBracket),
		VirtualKeyCode::Period     => Ok(Key::Period),
		VirtualKeyCode::RBracket   => Ok(Key::RightBracket),
		VirtualKeyCode::Semicolon  => Ok(Key::Semicolon),
		VirtualKeyCode::Slash      => Ok(Key::Slash),
		VirtualKeyCode::Tab        => Ok(Key::Tab),
		VirtualKeyCode::Subtract   => Ok(Key::Minus),

		_ => Err(())
	}
}
