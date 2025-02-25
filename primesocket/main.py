"""
Main script to start the UDP server.

This module initializes and runs the UDP server implemented in Rust.
"""
import asyncio

import primesocket_core


async def run_server():
    """
    Start the UDP server.

    This function initializes the server and keeps it running.
    """
    primesocket_core.start_server(8080, start=0, end=100000000, step=10)

if __name__ == "__main__":
    asyncio.run(run_server())
