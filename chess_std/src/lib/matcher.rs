

trait Matchable {

}

#[derive(Debug)]
struct Match<T, S> {
	value: T,
	rest: S,
}

trait MatchableFromString<T> {
	// if self is a prefix to source
	fn match_from_str<'a>(&self, source: &'a str) -> Option<Match<T, &'a str>>;
}

// TODO.optimize: self + return to be str slice instead
impl MatchableFromString<String> for String {
	fn match_from_str<'a>(&self, source: &'a str) -> Option<Match<String, &'a str>> {
		if source.starts_with(self) {
			let len = self.len();
			Some(Match {
				value: self.to_owned(),
				rest: &source[len..],
			})
		} else { None }
	}
}

struct DecimalDigitString;
// TODO.optimize: return u8 instead with safe unsafe converter to char/string/int
impl MatchableFromString<String> for DecimalDigitString {
	fn match_from_str<'a>(&self, source: &'a str) -> Option<Match<String, &'a str>> {
		const RADIX: u32 = 10;
		// TODO: see MatchableDigit
		source.chars().next()
		.filter(|c| c.is_digit(RADIX))
		.map(|c| {
			Match {
				value: c.to_string(),
				rest: &source[c.len_utf8()..],
			}
		})
	}
}

struct MatchableDigit {radix: u32}
impl MatchableFromString<u32> for MatchableDigit {
	fn match_from_str<'a>(&self, source: &'a str) -> Option<Match<u32, &'a str>> {
		source.chars().next()
		.map(|c| {
			let digit = c.to_digit(self.radix);
			digit.map(|value| Match {
				value,
				rest: &source[c.len_utf8()..],
			})
		}).flatten()
	}
}


type FF<T> = dyn MatchableFromString<T>;
struct MatchableOr<T>(Vec<Box<FF<T>>>);
impl<T> MatchableFromString<T> for MatchableOr<T> {
	fn match_from_str<'a>(&self, source: &'a str) -> Option<Match<T, &'a str>> {
		self.0.iter()
			.map(Box::as_ref)
			.map(|x| x.match_from_str(source))
			.filter(|x| x.is_some())
			.next()
			.flatten()
	}
}


#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn single_char() {
		if "b".to_owned().match_from_str("abc").is_some() {panic!("only match from start");}
		let m = "a".to_owned().match_from_str("abc").unwrap();
		assert_eq!(m.value, "a");
		assert_eq!(m.rest, "bc");
	}

	#[test]
	fn digit_string() {
		if DecimalDigitString.match_from_str("a1").is_some() {panic!()};
		let m = DecimalDigitString.match_from_str("1ab2").unwrap();
		assert_eq!(m.value, "1");
		assert_eq!(m.rest, "ab2");
	}

	#[test]
	fn matchable_digit() {
		if (MatchableDigit{radix: 10}).match_from_str("a1").is_some() {panic!()};
		let m = MatchableDigit{radix: 10}.match_from_str("1ab2").unwrap();
		assert_eq!(m.value, 1);
		assert_eq!(m.rest, "ab2");
	}

	fn or_single_char() {
		// let a = "a";
		// let b = &a;
		// let c = b.to_string();
		// let v = vec![Box::new(MatchableDigit{radix: 10})];
		// let o = MatchableOr(v);
		// let pattern = MatchableOr(["1", "2"].iter().map(|x| x.to_string()).map(Box::new).collect::<Vec<_>>());
		// pattern.match_from_str("1")
	}

	fn single_int() {

	}
}
