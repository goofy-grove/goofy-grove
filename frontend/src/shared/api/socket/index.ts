import { io } from 'socket.io-client';

export const socket = io(`${import.meta.env.VITE_SOCKET_URL}/v1`, {
  transports: ['websocket'],
  autoConnect: false,
});

export const updateSocketAuth = (token: string | null) => {
  if (token) {
    socket.auth = { token };

    if (socket.connected) {
      socket.disconnect();
      socket.connect();
    } else {
      socket.connect();
    }
  } else {
    socket.disconnect();
  }
};
