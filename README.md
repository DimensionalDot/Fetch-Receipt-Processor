# Receipt Processor

This repository contains my submission for a receipt processor take-home exercise from Fetch.
I look forward to reviewing my code with whoever is reading this!

## Running The Server

First load the docker image with:

`docker load -i receipt-processor-image.tar`

Then run it with:

`docker run --init --rm -p 127.0.0.1:<port>:3030 receipt-processor`

Where `<port>` is the local port you wish the server to run on.
If you wish to invoke the service manually, use the command:

`docker run -it --init --rm -p 127.0.0.1:<port>:3030 receipt-processor bash`

Now, within the container you can run `receipt-processor --help` to view available options.
Of course, if you then specify a port other than the default of `3030`, you will need to change which port you publish when running the container (via `-p <local port>:<container port>`).

## Rebuilding The Docker Image

If you encounter any issues using the pre-built Docker image you can rebuild it using the
Dockerfile contained in this repository. To do so, run the following commands from the directory root:

`docker build -t receipt-processor .`
