import asyncio
import primesocket_core


async def run_server():
    """
    Start the UDP server.

    This function initializes the server and keeps it running.
    """
    try:
        primesocket_core.start_server(9999, end=100000, verbose=0)
    except Exception as e:
        print(f"Error starting server: {e}")


if __name__ == "__main__":
    asyncio.run(run_server())
