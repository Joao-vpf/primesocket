import asyncio
import primesocket_core


async def run_client(ip: str, port: int):
    try:
        primesocket_core.start_client(ip, port, verbose=1)
    except Exception as e:
        print(f"Error starting server: {e}")


if __name__ == "__main__":
    server_ip = "127.0.0.1"
    server_port = 9999

    asyncio.run(run_client(server_ip, server_port))
