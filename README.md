# Receipt Processor

This repository contains my submission for a receipt processor take-home exercise from Fetch.
I look forward to reviewing my code with whoever is reading this!

## Running The Server

Using the Dockerfile contained in this repository run the following commands from the directory root:

`docker build -t receipt-processor .`

`docker run --init --rm -p <port>:3030 receipt-processor`

Where `<port>` is the local port you wish the server to run on.
If you wish to invoke the service manually, use the command:

`docker run -it --init --rm -p <port>:3030 receipt-processor bash`

Now, within the container you can run `receipt-processor --help` to view available options.
Of course, if you then specify a port other than the default of `3030`, you will need to change which port you publish when running the container (via `-p <local port>:<container port>`).
