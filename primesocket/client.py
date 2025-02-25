"""Run the client side of the PrimeSocket protocol."""
import asyncio

import primesocket_core


async def run_client(ip: str, port: int):
    """
    Runs the client side of the PrimeSocket protocol.

    Parameters
    ----------
    ip : str
        The IP address of the server to connect to.
    port : int
        The port number to connect to.
    """
    try:
        primesocket_core.start_client(ip, port, verbose=1)
    except Exception as e:
        print(f"Error starting client: {e}")


if __name__ == "__main__":
    asyncio.run(run_client("127.0.0.1", 9999))
