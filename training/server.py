import socket 

class TrainingServer: 

    def __init__(
            self, 
            hostname, 
            port
    ):
        self.hostname = hostname
        self.port = port

        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def connect(self): 
        self.socket.bind((self.hostname, self.port))
        print(f"Listening on {self.hostname} on port {self.port}")
        self.socket.listen()

    def listen(self, callback): 
        conn, addr = self.socket.accept()
        with conn : 
            chunkz = list()
            while True : 
                # Total bytes are number of squares for the 2 players
                # One byte if gradient is required or not
                data = conn.recv(64 * 2) 
                if not data : 
                    callback(chunkz)
                    break
                chunkz.append(data)

if __name__ == '__main__' : 

    server = TrainingServer('0.0.0.0', 65432)

    def callback(data) : 
        print(data)

    server.connect()

    while 1: 
        server.listen(callback)
