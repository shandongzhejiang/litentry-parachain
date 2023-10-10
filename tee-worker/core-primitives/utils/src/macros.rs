#[macro_export]
macro_rules! if_production_or {
	($prod_variant:expr, $non_prod_variant:expr) => {
		if cfg!(feature = "production") {
			$prod_variant
		} else {
			$non_prod_variant
		}
	};
}

#[macro_export]
macro_rules! if_not_production {
	($expression:expr) => {
		if cfg!(not(feature = "production")) {
			$expression
		}
	};
}