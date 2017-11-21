
use parse::Parseable;

pub struct Triple<T: Parseable>(pub T, pub T, pub T);
pub struct Sextuple<T: Parseable>(pub T, pub T, pub T, pub T, pub T, pub T);
