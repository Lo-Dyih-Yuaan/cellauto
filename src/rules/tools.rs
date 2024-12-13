#[inline(always)]
pub fn u32_print_str(n: u32) -> String {
	if n < 26 {
		std::char::from_u32(0x41u32+n).unwrap().to_string()
	} else {
		format!("[{}]", n)
	}
}

/**
 * 用于计数。
 * `$d`必为`$`，以解决宏内`$`将被转义的问题。
 * `$m`为统计对象；`$e`为待统计对象。
 * 返回一个长度与`$m`相同的`usize`数组。
 * 匹配使用`==`。
 */
macro_rules! count {
	(@nest $($body:tt)*) => {
		macro_rules! __with_dollar_sign { $($body)* }
		__with_dollar_sign!($);
	};
	(@index) => {0};
	(@index $e:tt) => {1};
	(@index $ef:tt, $($es:tt),+) => {1 + count!{@index $($es),+}};
	(@if $c:expr, $e:expr, $m:pat if $($os:pat),*) => {
		if let $m = $e {$c[count!{@index $($os),*}] += 1;}
	};
	(@if $c:expr, $e:expr, $m:pat, $($ms:pat),+ if $($os:pat),*) => {
		if let $m = $e {$c[count!{@index $($os),*}] += 1;}
		else {count!{@if $c, $e, $($ms),+ if $($os,)* $m}}
	};
	($($m:pat),+ in $($e:expr),*) => {{
		let mut temp: [usize; count!(@index $($m),+)] =
			[0; count!(@index $($m),+)];
		count!{@nest ($d:tt) => {
			macro_rules! __count {
				($d arg:expr) => {
					count!{@if temp, $d arg, $($m),+ if}
				};
			}
		}}
		$(__count!($e);)*
		temp
	}};
}

macro_rules! is_exist {
	($($p:pat)|+ in $i:expr) => {matches!($i, $($p)|+)};
	($($p:pat)|+ in $i:expr, $($is:expr),*) =>
		{matches!($i, $($p)|+) || is_exist!($($p)|+ in $($is),*)};
}