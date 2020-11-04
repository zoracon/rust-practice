The sample task will include parsing certificates in Go and validating a field. We’ll ask for a unittest with good test coverage.

Here’s the task: **Write a command-line tool** that takes a filename as its argument. It should **read the file and parse it as a PEM-formatted certificate**. For **each DNS name** in that certificate’s Subject Alternative Name field, it should check if the **DNS name is a valid hostname** in preferred name syntax from RFC 1035 (https://tools.ietf.org/html/rfc1035#section-2.3.1)*. It should then print to stdout one line for each name. That line should contain the name (with any newlines backslash-escaped), followed by a colon, a space and:

    “valid” if it’s a valid hostname, or
    “invalid” if it’s not.

If there is an error reading the PEM-formatted certificate from local disk or parsing it, your tool should print the filename, followed by a colon, a space, and an informative error message to stderr. It should then exit with status 42.

Your code should be clear and easy to read. It should be broken up into functions of appropriate size, and the functions and package should be commented in godoc style. The code should be formatted according to go fmt and should pass go vet cleanly. It should build and run correctly on the latest released version of Go.

**There should be a unittest**, and the unittest should have good coverage.

When you’re done, **bundle up your files into a .tar.gz or .zip file and reply to this email with the files attached**. The files should be under a “worksample/” directory in the archive you send. We should be able to unpack it and run go run ./worksample file.pem and go test ./worksample. If you have used a language other than Go, make sure you include instructions on how to compile (if necessary), execute, and test your code.

*Note that off-the-shelf DNS validators don’t validate this exact syntax. You should write your own.

Thanks for taking the time to do this, and we look forward to reading your code!