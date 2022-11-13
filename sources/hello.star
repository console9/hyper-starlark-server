
def main (_request) :
	
	_method, _uri, _headers, _body = _request
	_path, _query = _uri
	
	if True :
		
		_status = 200
		_headers = {
				"x-header-single" : "value",
				"x-header-multiple" : ["value-1", "value-2"],
			}
		_body = "OK"
		_response = (_status, _headers, _body)
		
	else :
		
		_response = "hello world!"
	
	return _response

