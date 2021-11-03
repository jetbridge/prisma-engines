initSidebarItems({"externcrate":[["byteorder",""],["either",""]],"fn":[["any","Parses any token."],["attempt","`attempt(p)` behaves as `p` except it acts as if the parser hadn’t consumed any input if `p` fails after consuming input. (alias for `try`)"],["between","Parses `open` followed by `parser` followed by `close`. Returns the value of `parser`."],["chainl1","Parses `p` 1 or more times separated by `op`. The value returned is the one produced by the left associative application of the function returned by the parser `op`."],["chainr1","Parses `p` one or more times separated by `op`. The value returned is the one produced by the right associative application of the function returned by `op`."],["choice","Takes a tuple, a slice or an array of parsers and tries to apply them each in order. Fails if all the parsers fails or if an applied parser consumes input before failing."],["count","Parses `parser` from zero up to `count` times."],["count_min_max","Parses `parser` from `min` to `max` times (including `min` and `max`)."],["env_parser","Constructs a parser out of an environment and a function which needs the given environment to do the parsing. This is commonly useful to allow multiple parsers to share some environment while still allowing the parsers to be written in separate functions."],["eof","Succeeds only if the stream is at end of input, fails otherwise."],["from_str","Takes a parser that outputs a string like value (`&str`, `String`, `&[u8]` or `Vec<u8>`) and parses it using `std::str::FromStr`. Errors if the output of `parser` is not UTF-8 or if `FromStr::from_str` returns an error."],["look_ahead","`look_ahead(p)` acts as `p` but doesn’t consume input on success."],["many","Parses `p` zero or more times returning a collection with the values from `p`."],["many1","Parses `p` one or more times returning a collection with the values from `p`."],["none_of","Extract one token and succeeds if it is not part of `tokens`."],["not_followed_by","Succeeds only if `parser` fails. Never consumes any input."],["one_of","Extract one token and succeeds if it is part of `tokens`."],["optional","Parses `parser` and outputs `Some(value)` if it succeeds, `None` if it fails without consuming any input. Fails if `parser` fails after having consumed some input."],["parser","Wraps a function, turning it into a parser."],["position","Parser which just returns the current position in the stream."],["satisfy","Parses a token and succeeds depending on the result of `predicate`."],["satisfy_map","Parses a token and passes it to `predicate`. If `predicate` returns `Some` the parser succeeds and returns the value inside the `Option`. If `predicate` returns `None` the parser fails without consuming any input."],["sep_by","Parses `parser` zero or more time separated by `separator`, returning a collection with the values from `p`."],["sep_by1","Parses `parser` one or more time separated by `separator`, returning a collection with the values from `p`."],["sep_end_by","Parses `parser` zero or more times separated and ended by `separator`, returning a collection with the values from `p`."],["sep_end_by1","Parses `parser` one or more times separated and ended by `separator`, returning a collection with the values from `p`."],["skip_count","Parses `parser` from zero up to `count` times skipping the output of `parser`."],["skip_count_min_max","Parses `parser` from `min` to `max` times (including `min` and `max`) skipping the output of `parser`."],["skip_many","Parses `p` zero or more times ignoring the result."],["skip_many1","Parses `p` one or more times ignoring the result."],["token","Parses a character and succeeds if the character is equal to `c`."],["tokens","Parses multiple tokens."],["tokens2","Parses multiple tokens."],["try","`try(p)` behaves as `p` except it acts as if the parser hadn’t consumed any input if `p` fails after consuming input."],["unexpected","Always fails with `message` as an unexpected error. Never consumes any input."],["unexpected_any","Always fails with `message` as an unexpected error. Never consumes any input."],["value","Always returns the value `v` without consuming any input."]],"macro":[["choice","Takes a number of parsers and tries to apply them each in order. Fails if all the parsers fails or if an applied parser consumes input before failing."],["opaque","Convenience macro over `opaque`."],["parser","Declares a named parser which can easily be reused."],["struct_parser","Sequences multiple parsers and builds a struct out of them."]],"mod":[["easy","Stream wrapper which provides an informative and easy to use error type."],["error","Error types and traits which define what kind of errors combine parsers may emit"],["parser","A collection of both concrete parsers as well as parser combinators."],["stream","Traits and implementations of arbitrary data streams."]],"trait":[["ParseError","Trait which defines a combine parse error."],["Parser","By implementing the `Parser` trait a type says that it can be used to parse an input stream into the type `Output`."],["Positioned","A type which has a position."],["RangeStream","A `RangeStream` is an extension of `Stream` which allows for zero copy parsing."],["RangeStreamOnce","A `RangeStream` is an extension of `StreamOnce` which allows for zero copy parsing."],["Stream","A stream of tokens which can be duplicated"],["StreamOnce","`StreamOnce` represents a sequence of items that can be extracted one by one."]],"type":[["ConsumedResult","A `Result` type which has the consumed status flattened into the result. Conversions to and from `std::result::Result` can be done using `result.into()` or `From::from(result)`"],["ParseResult","A type alias over the specific `Result` type used by parsers to indicate whether they were successful or not. `O` is the type that is output on success. `I` is the specific stream type used in the parser."]]});