"""
Client for sending sieve computation requests to the server.

This script simulates the execution of the Sieve of Eratosthenes and
communicates with the server to update the computation state.
"""
import json
import socket

SERVER_ADDRESS = ('127.0.0.1', 8080)


def send_udp_request(request_data):
    """Sends a UDP request to the server and returns the response."""
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as sock:
        sock.settimeout(10)

        message = json.dumps(request_data).encode('utf-8')
        sock.sendto(message, SERVER_ADDRESS)
        print(f"📤 Sent: {request_data}")

        try:
            data, _ = sock.recvfrom(4096)
            response = json.loads(data.decode('utf-8'))
            print(f"📩 Received: {response}")
            return response
        except socket.timeout:
            print("⚠️ No response received (timeout)")
            return None


def run_sieve_simulation():
    """Simulates the sieve algorithm and updates the server state."""
    print("🔄 Starting Sieve of Eratosthenes simulation...")

    # Step 1: Start the computation
    response = send_udp_request({"task": "start"})
    if response is None or response["task"] != "range":
        print("❌ Failed to start sieve computation")
        return

    start = response["start"]
    end = response["end"]
    sieve = response["sieve"]

    # Step 2: Execute the sieve logic
    print(f"🔍 Running sieve for range: {start} to {end}")
    for i in range(2, len(sieve)):
        if sieve[i]:  # If it's prime
            for j in range(i * i, len(sieve), i):
                sieve[j] = 0  # Mark as non-prime

    # Step 3: Send updated sieve to server
    send_udp_request({"task": "save", "end": end, "sieve": sieve})

    # Step 4: Fetch the current state
    send_udp_request({"task": "fetch"})


if __name__ == "__main__":
    run_sieve_simulation()
