import asyncio
import primesocket_core


async def run_client(ip: str, port: int, timeout: int = 1):
    client_task = asyncio.create_task(primesocket_core.start_client(ip, port))

    try:
        await asyncio.wait_for(client_task, timeout)
    except asyncio.TimeoutError:
        print(f"⚠️ Timeout atingido após {timeout} segundo(s). Cliente cancelado.")
        client_task.cancel()
        await client_task


if __name__ == "__main__":
    # IP e porta do servidor
    server_ip = "127.0.0.1"
    server_port = 8080

    # Executando o cliente
    asyncio.run(run_client(server_ip, server_port))
