
def main (_method, _headers, _path, _query) :
	print (repr (_method), repr (_path), repr (_query))
	print (repr (_headers))
	return "hello world!"

