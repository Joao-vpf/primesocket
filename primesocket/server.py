"""Server side for the PrimeSocket library.

This module provides the PrimeServer class and a main function for starting
the server via CLI.
"""

import argparse

import primesocket_core


class PrimeServer:  # pylint: disable=R0903
    """Binding for the Rust-based UDP prime server."""

    def __init__(
        self,
        port: int = 9999,
        end: int = 9999,
        verbose: int = 0
    ):
        """
        Initialize the PrimeServer instance.

        Parameters
        ----------
        port : int
            The UDP port where the server will listen.
        end : int
            The upper bound of the number range to process.
        verbose : int, optional
            Verbosity level (default: 0 - no logs, higher values for
            more details).
        """
        self.port = port
        self.end = end
        self.verbose = verbose

    def start(self):
        """
        Start the Rust-based UDP server.

        This method initializes the server and begins listening for incoming
        client requests. The server handles computations for prime detection
        asynchronously.

        Raises
        ------
        ValueError
            If the `end` parameter is not provided or is invalid.
        """
        try:
            primesocket_core.start_server(
                self.port,
                self.end,
                self.verbose
            )
        except ValueError as e:
            print(f"[Error] Failed to start server: {e}")


def main():
    """CLI entry point for running the PrimeServer."""
    parser = argparse.ArgumentParser(description="PrimeSocket Server")
    parser.add_argument(
        "--port",
        type=int,
        required=True,
        help="UDP port to bind"
    )
    parser.add_argument(
        "--end",
        type=int,
        required=True,
        help="Upper bound for prime computation"
    )
    parser.add_argument(  # pylint: disable=R0801
        "--verbose",
        type=int,
        default=0,
        help="Verbosity level"
    )

    args = parser.parse_args()

    server = PrimeServer(
        port=args.port,
        end=args.end,
        verbose=args.verbose
    )
    server.start()


if __name__ == "__main__":
    main()
