"""
Script for running multiple clients concurrently.

This module spawns multiple threads, each executing a client that
communicates with the server using UDP.
"""
import threading
import time

from client_sieve import run_sieve_simulation

# Define o número de clientes concorrentes
NUM_CLIENTS = 5


def start_client():
    """Function to start an individual client thread."""
    run_sieve_simulation()


if __name__ == "__main__":
    print(f"🚀 Starting {NUM_CLIENTS} concurrent clients...")

    threads = []

    for _ in range(NUM_CLIENTS):
        thread = threading.Thread(target=start_client)
        threads.append(thread)
        thread.start()
        # Pequeno delay para não sobrecarregar o servidor de uma vez
        time.sleep(0.5)

    for thread in threads:
        thread.join()

    print("✅ All clients finished execution.")
