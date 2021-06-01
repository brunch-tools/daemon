import 'dart:developer';
import 'dart:io';

final String HOST = "localhost";
final int PORT = 15744;

void main(List<String> arguments) {
  var webSocketHandler = WebSocketHandler();
  print('brunch-tools-daemon: Attempted to bind to '+HOST+':'+PORT.toString()+'!');
  HttpServer.bind(HOST, PORT).then((server) => {
    server.listen((request) {
      WebSocketTransformer.upgrade(request).then((socket) => {
        webSocketHandler.newRequest(socket)
      });
    })
  });
}

class WebSocketHandler {
  void newRequest(WebSocket socket) {
    log('Incoming connection ${socket.hashCode}');
    socket.listen((event) {
      socket.add(event);
      socket.close(1);
    });
  }

}