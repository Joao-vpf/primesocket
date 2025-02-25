"""Runs multiple clients that connect to the same server."""
import multiprocessing

import primesocket_core


def run_client(ip: str, port: int, client_id: int):
    """Runs a client that connects to the server.

    Parameters
    ----------
    ip : str
        The IP address of the server.
    port : int
        The port to connect to.
    client_id : int
        The ID of the client.
    """
    try:
        print(f"Client {client_id} starting...")
        primesocket_core.start_client(ip, port, verbose=0)
        print(f"Client {client_id} finished.")
    except Exception as e:
        print(f"Error in Client {client_id}: {e}")


def run_multiple_clients(ip: str, port: int, num_clients: int):
    """Runs multiple clients concurrently using multiprocessing.

    Parameters
    ----------
    ip : str
        The IP address of the server.
    port : int
        The port to connect to.
    num_clients : int
        The number of clients to run.
    """
    processes = []
    for client_id in range(1, num_clients + 1):
        process = multiprocessing.Process(
            target=run_client, args=(ip, port, client_id))
        process.start()
        processes.append(process)

    for process in processes:
        process.join()


if __name__ == "__main__":
    run_multiple_clients("127.0.0.1", 9999, 10)
