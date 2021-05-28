import 'dart:developer';
import 'dart:io';

final String HOST = "localhost";
final int PORT = 15744;

void main(List<String> arguments) {
  var webSocketHandler = WebSocketHandler();
  print('brunch-tools-daemon: Attempted to bind to '+HOST+':'+PORT.toString()+'!');
  HttpServer.bind(HOST, PORT).then((server) => {
    server.listen((request) {
      print('brunch-tools-daemon: Bound, listening to websocket requests.');
      WebSocketTransformer.upgrade(request).then((socket) => {
        webSocketHandler.newRequest(socket)
      });
    })
  });
}

class WebSocketHandler {
  Map<String, WebSocket> _users = {};

  void newRequest(WebSocket socket) {
    log('Incoming connection ${socket.hashCode}');
    socket.listen((event) {

    });
  }

}