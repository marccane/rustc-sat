fn main() { match todo!() {
(true, true, _, _) => {}
(_, true, true, _) => {}
(_, _, true, true) => {}
(true, _, _, true) => {}
(false, _, false, _) => {}
(_, false, _, false) => {}
} }
