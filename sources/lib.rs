



mod star {
	pub use ::starlark::{
		
		syntax::AstModule,
		syntax::Dialect,
		syntax::DialectTypes,
		
		environment::Globals,
		environment::Module,
		
		eval::Evaluator,
		
	};
}


use ::hyper_simple_server as hss;

use ::vrl_errors::*;

pub use hss::MainResult;




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
			
			let _empty_module = star::Module::new ();
			let mut _evaluator = star::Evaluator::new (&_empty_module);
			let _main = _main.clone ();
			let _main = _main.value ();
			let _outcome = _evaluator.eval_function (_main, &[], &[]) .unwrap ();
			
			let _body = format! ("{}", _outcome);
			
			let _body = hss::Body::from (_body);
			let mut _response = hss::Response::new (_body);
			Ok (_response)
		};
	
	let _handler = hss::HandlerFnSync::from (_handler);
	let _handler = ::std::sync::Arc::new (_handler);
	
	return hss::main_with_handler (_handler, None, None);
}

