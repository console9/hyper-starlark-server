



mod star {
	pub use ::starlark::{
		
		syntax::AstModule,
		syntax::Dialect,
		syntax::DialectTypes,
		
		environment::Globals,
		environment::Module,
		
		eval::Evaluator,
		
		values::Value,
		values::dict::Dict,
		values::Heap,
		
		collections::SmallMap,
		
	};
}

use ::starlark::{
		
		values::AllocValue as _,
		values::ValueLike as _,
		
	};




use ::hyper_simple_server as hss;

use hss::{
		
		ResponseExtBuild as _,
		
	};

pub use hss::MainResult;




use ::vrl_errors::*;








pub fn main () -> MainResult {
	
	
	
	
	let _code = include_str! ("./hello.star");
	let _code = _code.to_owned ();
	
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
	
	let _ast = star::AstModule::parse ("__main__", _code, &_dialect) .anyerr () .else_wrap (0xd8d5a76c) ?;
	
	let _globals = star::Globals::extended ();
	
	let _module = star::Module::new ();
	
	let mut _evaluator = star::Evaluator::new (&_module);
	
	let _outcome = _evaluator.eval_module (_ast, &_globals) .anyerr () .else_wrap (0x90367326) ?;
	assert! (_outcome.is_none ());
	
	let _module = _module.freeze () .anyerr () .else_wrap (0xcb296b53) ?;
	
	let _main = _module.get ("main") .anyerr () .else_wrap (0x53cc39d4) ?;
	
	let _handler = move |_request : hss::Request<hss::Body>| {
			
			let _request_module = star::Module::new ();
			
			let _heap = _request_module.heap ();
			
			let _request_method_value = _heap.alloc_str (_request.method () .as_str ()) .to_value ();
			let _request_path_value = _heap.alloc_str (_request.uri () .path ()) .to_value ();
			let _request_query_value = if let Some (_query) = _request.uri () .query () {
					_heap.alloc_str (_query).to_value ()
				} else {
					star::Value::new_none ()
				};
			
			let _request_headers_value = {
					let _headers_http = _request.headers ();
					let mut _headers_map = star::SmallMap::with_capacity (_headers_http.keys_len ());
					for _name_http in _headers_http.keys () {
						let _values_http = _headers_http.get_all (_name_http) .into_iter () .collect::<Vec<_>> ();
						let _name_value = _heap.alloc_str (_name_http.as_str ()) .to_value ();
						let _name_hashed = _name_value.get_hashed () .anyerr () .else_wrap (0x676d34f9) ?;
						fn _http_to_star <'v> (_http : &hss::HeaderValue, _heap : &'v star::Heap) -> star::Value<'v> {
							let _http = String::from_utf8_lossy (_http.as_bytes ());
							_heap.alloc_str (&_http) .to_value ()
						}
						if _values_http.len () == 1 {
							let _value = _http_to_star (&_values_http[0], _heap);
							_headers_map.insert_hashed (_name_hashed, _value);
						} else {
							let _values = _heap.alloc_list_iter (_values_http.into_iter () .map (|_http| _http_to_star (_http, _heap))) .to_value ();
							_headers_map.insert_hashed (_name_hashed, _values);
						}
					}
					let _headers_dict = star::Dict::new (_headers_map);
					_headers_dict.alloc_value (_heap)
				};
			
			let mut _evaluator = star::Evaluator::new (&_request_module);
			let _main = _main.clone ();
			let _main = _main.value ();
			let _outcome = _evaluator.eval_function (_main, &[_request_method_value, _request_headers_value, _request_path_value, _request_query_value], &[]) .unwrap ();
			
			if let Some (_text) = _outcome.unpack_str () {
				let _text = _text.to_owned ();
				let _response = hss::Response::new_200_with_text (_text);
				return Ok (_response);
			}
			
			let _body = format! ("{}", _outcome);
			let _body = hss::Body::from (_body);
			let _response = hss::Response::new_200_with_text (_body);
			return Ok (_response);
		};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	let _handler = ::std::sync::Arc::new (_handler);
	
	return hss::main_with_handler (_handler, None, None);
}

