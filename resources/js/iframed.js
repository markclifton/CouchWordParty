function connectWebSocket(window, socketKey, onopenCallback, onmessageCallback) {
    if (window.socketsCollection.exists(socketKey)) return;

    var newSocketData = {
        url: "ws://192.168.1.2:7777/ws",
        onopen: onopenCallback.toString(),
        onclose: function (e, key, ws) {
            //alert("Connection is closed...");
        },
        onmessage: onmessageCallback.toString(),
    };

    window.socketsCollection.add(socketKey, newSocketData);
}

function sendWord(window) {
    window.socketsCollection.getSocket("active").send("123");
}