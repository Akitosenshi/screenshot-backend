## screenshot service

- screenshots will be processed by the server
- screenshots should be send to the server via REST api as raw binary data

## stories
- server has to save screenshots
- screenshots should be accessible via a url
- must be protected against unauthorized access (JWT , z.b.) / basic auth: "Header: {"Basic-Auth("username:"admin", password:"admin"")"}" / "headers: "authorization: bearer: {eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyLCJleHAiOjk5ODE5MTkxOSwicGVyc29uYWxfc2NvcGUiOjF9.V7qpdb5h_c54-IH90DQ2VDh8_-vIXMLtM97cugQ7B8w}"
- uploaded screenshot must be named by the server(sha256 will be used)
- Screenshot url has to be returned in the http response
- Screenshots can be managed by the user that created it(public/private eg. for specific users, login required)

### questions
- how does the server sve the screenshot?
	- via rest api call & file write

#### projection
- screenshot client with management capabilities + maybe configuration capabilities for 3rd party services
