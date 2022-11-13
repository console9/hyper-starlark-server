



mod star {
	pub use ::starlark::{
		
		syntax::AstModule,
		syntax::Dialect,
		syntax::DialectTypes,
		
		environment::Globals,
		environment::Module,
		environment::FrozenModule,
		
		eval::Evaluator,
		
		values::Value,
		values::OwnedFrozenValue,
		values::Heap,
		values::list::List,
		values::dict::Dict,
		values::tuple::Tuple,
		
		collections::SmallMap,
		
	};
}

use ::starlark::{
		
		values::AllocValue as _,
		values::ValueLike as _,
		
	};




use ::hyper_simple_server as hss;

use hss::{
		
		ResponseExt as _,
		ResponseExtBuild as _,
		
	};

pub use hss::MainResult;




use ::vrl_errors::*;
use ::vrl_preludes::std_plus_extras::*;








pub fn main () -> MainResult {
	
	let mut _source_path = String::from ("");
	
	let _prepare_extensions = [
			hss::CliArgument::String (&mut _source_path, "--handler", "path to Starlark handler source"),
		];
	let _configuration = hss::prepare_configuration_with_extensions (None, _prepare_extensions, None) ?;
	
	let _source = if _source_path.is_empty () {
			let _source = include_str! ("./debug.star");
			Cow::Borrowed (_source)
		} else {
			let _source_path = Path::new (&_source_path);
			let _source_data = fs::read (_source_path) .else_wrap (0x9b128523) ?;
			let _source = String::from_utf8 (_source_data) .else_wrap (0x8d89a350) ?;
			Cow::Owned (_source)
		};
	
	let _handler = StarlarkHandler::new (&_source) ?;
	let _handler = Arc::new (_handler);
	
	return hss::run_with_handler (_handler, _configuration);
}








pub struct StarlarkHandler {
	
	pub module : star::FrozenModule,
	pub main : star::OwnedFrozenValue,
}


impl StarlarkHandler {
	
	
	pub fn new (_source : &str) -> MainResult<StarlarkHandler> {
		
		let _source = _source.to_owned ();
		
		let _dialect = star::Dialect {
				
				enable_def : true,
				enable_lambda : true,
				enable_keyword_only_arguments : true,
				enable_types : star::DialectTypes::Enable,
				enable_tabs : true,
				
				enable_top_level_stmt : false,
				
				enable_load : false,
				enable_load_reexport : false,
			};
		
		let _ast = star::AstModule::parse ("__main__", _source, &_dialect) .anyerr () .else_wrap (0xd8d5a76c) ?;
		
		let _globals = star::Globals::extended ();
		
		let _module = star::Module::new ();
		
		let mut _evaluator = star::Evaluator::new (&_module);
		
		let _outcome = _evaluator.eval_module (_ast, &_globals) .anyerr () .else_wrap (0x90367326) ?;
		if ! _outcome.is_none () {
			fail! (0x45f44f7d, "invalid module evaluation value");
		}
		
		let _module = _module.freeze () .anyerr () .else_wrap (0xcb296b53) ?;
		
		let _main = _module.get ("main") .anyerr () .else_wrap (0x53cc39d4) ?;
		
		let _handler = StarlarkHandler {
				module : _module,
				main : _main,
			};
		
		return Ok (_handler);
	}
	
	
	pub fn handle (&self, mut _request_http : hss::Request<hss::Body>) -> hss::HandlerResult<hss::Response<hss::Body>> {
		
		let mut _request_body_http = hss::Body::empty ();
		swap (_request_http.body_mut (), &mut _request_body_http);
		
		// FIXME!
		// let _request_body_bytes = hss::hyper::body::to_bytes (_request_body_http) .await ? .to_vec ();
		let _request_body_bytes = Vec::new ();
		
		let _transaction_module = star::Module::new ();
		let _transaction_heap = _transaction_module.heap ();
		
		let _request_body_star = convert_body_bytes_to_star (_request_body_bytes, _transaction_heap) .else_wrap (0x8bb85596) ?;
		let _request_body_star = Some (_request_body_star);
		
		let _request_star = convert_request_http_to_star (&_request_http, _request_body_star, _transaction_heap) .else_wrap (0x8bd1df19) ?;
		
		let mut _transaction_evaluator = star::Evaluator::new (&_transaction_module);
		let _transaction_handler = self.main.value ();
		
		let _response_star = _transaction_evaluator.eval_function (_transaction_handler, &[_request_star], &[]) .anyerr () .else_wrap (0x6d5597ec) ?;
		
		let _response_http = convert_response_star_to_http (_response_star) .else_wrap (0xb9cd80b7) ?;
		
		return Ok (_response_http);
	}
}


impl hss::Handler for StarlarkHandler {
	
	type Future = hss::futures::future::Ready<hss::HandlerResult<hss::Response<hss::Body>>>;
	type ResponseBody = hss::Body;
	type ResponseBodyError = hss::hyper::Error;
	
	fn handle (&self, _request : hss::Request<hss::Body>) -> Self::Future {
		let _outcome = StarlarkHandler::handle (self, _request);
		return hss::futures::future::ready (_outcome);
	}
}








define_error! (pub ValueError, result : ValueResult, application : 0x78b7f6646e82a1fd95d053f985867d21, module : 0xb6eaaf88, type : 0xc0391b4b);




pub fn convert_request_http_to_star <'v> (_request_http : &hss::Request<hss::Body>, _body_star : Option<star::Value<'v>>, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _method_star = convert_method_http_to_star (_request_http.method (), _heap) ?;
	let _uri_star = convert_uri_http_to_star (_request_http.uri (), _heap) ?;
	let _headers_star = convert_headers_http_to_star (_request_http.headers (), _heap) ?;
	
	let _body_star = if let Some (_body_star) = _body_star {
			_body_star
		} else {
			star::Value::new_none ()
		};
	
	let _request_star = [_method_star, _uri_star, _headers_star, _body_star];
	let _request_star = _heap.alloc_tuple (&_request_star) .to_value ();
	
	return Ok (_request_star);
}




pub fn convert_response_star_to_http (_response_star : star::Value) -> ValueResult<hss::Response<hss::Body>> {
	
	if _response_star.is_none () {
		let _response_http = hss::Response::new_204 ();
		return Ok (_response_http);
	}
	
	if let Some (_string) = _response_star.unpack_str () {
		let _string = _string.to_owned ();
		let _response_http = hss::Response::new_200_with_text (_string);
		return Ok (_response_http);
	}
	
	if let Some (_response_star) = star::Tuple::from_value (_response_star) {
		return convert_response_tuple_to_http (_response_star);
	}
	
	fail! (0xf01c207a, "invalid http response value");
}


pub fn convert_response_tuple_to_http (_response_star : &star::Tuple) -> ValueResult<hss::Response<hss::Body>> {
	
	match _response_star.content () {
		
		[_status_star, _headers_star, _body_star] => {
			
			let _status_http = convert_status_star_to_http (_status_star.clone ()) ?;
			let _headers_http = convert_headers_star_to_http (_headers_star.clone ()) ?;
			let _body_http = convert_body_star_to_http (_body_star.clone ()) ?;
			
			let mut _response_http = hss::Response::new_with_status_and_body (_status_http, _body_http, None::<hss::HeaderValue>);
			_response_http.set_headers (_headers_http);
			
			return Ok (_response_http);
		}
		
		_ =>
			fail! (0x1f1f30a0, "invalid http response tuple size"),
	}
}




pub fn convert_method_http_to_star <'v> (_method_http : &hss::Method, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _string = _method_http.as_str ();
	
	let _method_star = _heap.alloc_str (&_string) .to_value ();
	
	return Ok (_method_star);
}



pub fn convert_uri_http_to_star <'v> (_uri_http : &hss::Uri, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _path_star = convert_uri_path_http_to_star (_uri_http, _heap) ?;
	let _query_star = convert_uri_query_http_to_star (_uri_http, _heap) ?;
	
	let _uri_star = [_path_star, _query_star];
	let _uri_star = _heap.alloc_tuple (&_uri_star) .to_value ();
	
	return Ok (_uri_star);
}


pub fn convert_uri_path_http_to_star <'v> (_uri_http : &hss::Uri, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _string = _uri_http.path ();
	
	let _path_star = _heap.alloc_str (&_string) .to_value ();
	
	return Ok (_path_star);
}


pub fn convert_uri_query_http_to_star <'v> (_uri_http : &hss::Uri, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _query_star = if let Some (_string) = _uri_http.query () {
			_heap.alloc_str (_string) .to_value ()
		} else {
			star::Value::new_none ()
		};
	
	return Ok (_query_star);
}




pub fn convert_status_star_to_http (_status_star : star::Value) -> ValueResult<hss::StatusCode> {
	
	let _code = _status_star.unpack_int () .else_wrap_with_message (0xa1be0502, "invalid http response status code") ?;
	let _code : u16 = _code.try_into () .else_wrap_with_message (0xf275627c, "invalid http response status code") ?;
	let _code = hss::StatusCode::from_u16 (_code) .else_wrap_with_message (0x491bad36, "invalid http response status code") ?;
	
	return Ok (_code);
}




pub fn convert_headers_http_to_star <'v> (_headers_http : &hss::HeaderMap, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let mut _headers_star = star::SmallMap::with_capacity (_headers_http.keys_len ());
	
	for _name_http in _headers_http.keys () {
		
		let _values_http = _headers_http.get_all (_name_http) .into_iter () .collect::<Vec<_>> ();
		
		let _name_star = convert_header_name_http_to_star (_name_http, _heap) ?;
		let _values_star = convert_header_values_http_to_star (&_values_http, _heap) ?;
		
		let _name_star_hashed = _name_star.get_hashed () .anyerr () .else_wrap (0x676d34f9) ?;
		_headers_star.insert_hashed (_name_star_hashed, _values_star);
	}
	
	let _headers_star = star::Dict::new (_headers_star);
	let _headers_star = _headers_star.alloc_value (_heap);
	
	return Ok (_headers_star);
}


pub fn convert_headers_star_to_http (_headers_star : star::Value) -> ValueResult<hss::HeaderMap> {
	
	if _headers_star.is_none () {
		return Ok (hss::HeaderMap::new ());
	}
	
	if let Some (_headers_star) = star::Dict::from_value (_headers_star) {
		let mut _headers_http = hss::HeaderMap::with_capacity (_headers_star.len () * 4/3);
		for (_name_star, _values_star) in _headers_star.iter () {
			let _name_http = convert_header_name_star_to_http (_name_star) ?;
			let _values_http = convert_header_values_star_to_http (_values_star) ?;
			for _value_http in _values_http.into_iter () {
				_headers_http.append (_name_http.clone (), _value_http);
			}
		}
		return Ok (_headers_http);
	}
	
	if let Some (_headers_star) = star::List::from_value (_headers_star) {
		let mut _headers_http = hss::HeaderMap::with_capacity (_headers_star.len () * 4/3);
		for _header_star in _headers_star.iter () {
			if let Some (_header_star) = star::Tuple::from_value (_header_star) {
				match _header_star.content () {
					[_name_star, _values_star] => {
						let _name_http = convert_header_name_star_to_http (_name_star.clone ()) ?;
						let _values_http = convert_header_values_star_to_http (_values_star.clone ()) ?;
						for _value_http in _values_http.into_iter () {
							_headers_http.append (_name_http.clone (), _value_http);
						}
					},
					_ =>
						fail! (0x9cc3d551, "invalid http header tuple"),
				}
			} else {
				fail! (0x23fe7bda, "invalid http header value");
			}
		}
		return Ok (_headers_http);
	}
	
	fail! (0x04d2f05a, "invalid http headers value");
}




pub fn convert_header_name_http_to_star <'v> (_name_http : &hss::HeaderName, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _string = _name_http.as_str ();
	
	let _name_star = _heap.alloc_str (&_string) .to_value ();
	
	return Ok (_name_star);
}


pub fn convert_header_name_star_to_http (_name_star : star::Value) -> ValueResult<hss::HeaderName> {
	
	if let Some (_string) = _name_star.unpack_str () {
		let _name_http = hss::HeaderName::from_str (_string) .else_wrap_with_message (0x98dc7519, "invalid http header name string") ?;
		return Ok (_name_http);
	}
	
	fail! (0xe12fb5e6, "invalid http header name value");
}




pub fn convert_header_value_http_to_star <'v> (_value_http : &hss::HeaderValue, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	let _string = _value_http.as_bytes ();
	let _string = String::from_utf8_lossy (_string);
	
	let _value_star = _heap.alloc_str (&_string) .to_value ();
	
	return Ok (_value_star);
}


pub fn convert_header_value_star_to_http (_value_star : star::Value) -> ValueResult<hss::HeaderValue> {
	
	if let Some (_string) = _value_star.unpack_str () {
		let _value_http = hss::HeaderValue::from_str (_string) .else_wrap_with_message (0x2879a4a6, "invalid http header value string") ?;
		return Ok (_value_http);
	}
	
	fail! (0x94314687, "invalid http header value value");
}




pub fn convert_header_values_http_to_star <'v> (_values_http : &[&hss::HeaderValue], _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	match _values_http {
		
		[_value_http] => {
			let _value_star = convert_header_value_http_to_star (_value_http, _heap) ?;
			return Ok (_value_star);
		}
		
		_ => {
			let mut _values_star = Vec::with_capacity (_values_http.len ());
			for _value_http in _values_http {
				let _value_star = convert_header_value_http_to_star (_value_http, _heap) ?;
				_values_star.push (_value_star);
			}
			let _values_star = _heap.alloc_list_iter (_values_star.into_iter ()) .to_value ();
			return Ok (_values_star);
		}
	}
}


pub fn convert_header_values_star_to_http (_values_star : star::Value) -> ValueResult<Vec<hss::HeaderValue>> {
	
	if let Some (_) = _values_star.unpack_str () {
		let _value_http = convert_header_value_star_to_http (_values_star) ?;
		let mut _values_http = Vec::new ();
		_values_http.push (_value_http);
		return Ok (_values_http);
	}
	
	if let Some (_values_star) = star::List::from_value (_values_star) {
		let mut _values_http = Vec::with_capacity (_values_star.len ());
		for _value_star in _values_star.iter () {
			let _value_http = convert_header_value_star_to_http (_value_star) ?;
			_values_http.push (_value_http);
		}
		return Ok (_values_http);
	}
	
	fail! (0x9ceae580, "invalid http header values value");
}




pub fn convert_body_bytes_to_star <'v> (_body_bytes : Vec<u8>, _heap : &'v star::Heap) -> ValueResult<star::Value<'v>> {
	
	if _body_bytes.is_empty () {
		let _body_star = star::Value::new_none ();
		return Ok (_body_star);
	}
	
	if let Ok (_string) = str::from_utf8 (&_body_bytes) {
		let _body_star = _heap.alloc_str (_string) .to_value ();
		return Ok (_body_star);
	}
	
	fail! (0x471b1c19, "unexpected http body binary");
}


pub fn convert_body_star_to_http (_body_star : star::Value) -> ValueResult<hss::Body> {
	
	if _body_star.is_none () {
		return Ok (hss::Body::empty ());
	}
	
	if let Some (_string) = _body_star.unpack_str () {
		let _string = _string.to_owned ();
		let _body_http = hss::Body::from (_string);
		return Ok (_body_http);
	}
	
	fail! (0x7fc983e4, "invalid http body value");
}

