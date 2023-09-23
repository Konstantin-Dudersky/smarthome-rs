Сервис подключается по каналу websocket к deconz-hub и пересылает сообщения в Redis.

Документация - https://dresden-elektronik.github.io/deconz-rest-doc/endpoints/websocket/

Пример сообщения от датчика открытия / закрытия:

```json
{
  "e": "changed",
  "id": "7",
  "r": "sensors",
  "state": {
    "lastupdated": "2023-09-23T09:25:38.851",
    "open": false
  },
  "t": "event",
  "uniqueid": "00:15:8d:00:03:21:44:8c-01-0006"
}
```
