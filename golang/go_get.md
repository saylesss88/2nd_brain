# go_get
#go

GO requires that the dependencies for a project be installed locally
on your machine as you develop and if they aren't you'll get an error
like this:
```go
ERROR github.com/pkg/name is not installed.
```
And to solve it run:
```go
go get github.com/pkg/name
```

The catch is that you must have done `go mod init` first.


