import asyncio
import primesocket_core


async def run_client(ip: str, port: int, client_id: int):
    """_summary_

    Parameters
    ----------
    ip : str
        _description_
    port : int
        _description_
    client_id : int
        _description_
    """    
    try:
        print(f"Cliente {client_id} iniciando...")
        primesocket_core.start_client(ip, port)
        print(f"Cliente {client_id} finalizado.")
    except Exception as e:
        print(f"Erro no Cliente {client_id}: {e}")


async def run_multiple_clients(ip: str, port: int, num_clients: int):
    """_summary_

    Parameters
    ----------
    ip : str
        _description_
    port : int
        _description_
    num_clients : int
        _description_
    """    
    tasks = []
    for client_id in range(1, num_clients + 1):
        tasks.append(run_client(ip, port, client_id))
    
    # Aguarda todos os clientes completarem
    await asyncio.gather(*tasks)


if __name__ == "__main__":
    """_summary_
    """    
    # IP e porta do servidor
    server_ip = "127.0.0.1"
    server_port = 8080
    num_clients = 5  # Número de clientes para testar
    
    # Executando múltiplos clientes
    asyncio.run(run_multiple_clients(server_ip, server_port, num_clients))
