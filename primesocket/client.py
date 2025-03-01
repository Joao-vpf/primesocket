import argparse
import primesocket_core


class PrimeClient:
    """
    A Python binding for the Rust-based UDP client that interacts with the server.
    """
    
    def __init__(self, ip: str="127.0.0.1", port: int=9999, verbose: int = 0, timeout: int = 120):
        """
        Initializes the PrimeClient instance.
        
        Parameters
        ----------
        ip : str
            The IP address of the server.
        port : int
            The UDP port where the server is listening.
        verbose : int, optional
            Verbosity level (default: 0 - no logs, higher values for more details).
        timeout : int, optional
            Timeout duration in seconds (default: 120 seconds).
        """        
        self.ip = ip
        self.port = port
        self.verbose = verbose
        self.timeout = timeout
    
    def start(self):
        """
        Starts the client and connects to the Rust-based UDP server.
        
        This method initializes the client and attempts to send requests to the server.
        The client communicates asynchronously and processes responses accordingly.
        
        Raises
        ------
        ValueError
            If the client fails to connect or send a request.
        """
        try:
            primesocket_core.start_client(self.ip, self.port, self.verbose, self.timeout)
        except ValueError as e:
            print(f"[Error] Failed to start client: {e}")

def main():
    parser = argparse.ArgumentParser(description="PrimeSocket Client")
    parser.add_argument("--ip", type=str, required=True, help="Server IP address")
    parser.add_argument("--port", type=int, required=True, help="UDP port to connect")
    parser.add_argument("--verbose", type=int, default=0, help="Verbosity level")
    parser.add_argument("--timeout", type=int, default=120, help="Timeout in seconds")

    args = parser.parse_args()
    
    client = PrimeClient(args.ip, args.port, args.verbose, args.timeout)
    client.start()

if __name__ == "__main__":
    main()