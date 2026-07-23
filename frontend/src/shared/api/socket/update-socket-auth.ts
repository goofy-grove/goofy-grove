import { socket } from './socket';

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
