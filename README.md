# Photochooser
Веб-приложения для упрощения процесса выбора фотографий заказчиками.
Проект представляет из себя веб-интерфейс и серверную часть, которая позволяет хранить фотографии, давать доступ определенным клиентам и получать информацию об их выборе.

## Запуск
Для работы приложения необходим Docker:
```sh
docker compose up -d
```


## Описание Userflow для разработчиков

1. Зарегистрировать фотографа и клиента
POST http://127.0.0.1:3000/sign_up
```json
{
  "username": "photographer",
  "email": "photo@photo.com",
  "password": "example"
}
```

```json
{
  "username": "client",
  "email": "client@client.com",
  "password": "example"
}
```
2. Залогинитсья и получить id пользователей
POST http://127.0.0.1:3000/login
```json
{
  "email": "photo@photo.com",
  "password": "example"
}
```

```json
{
  "email": "client@client.com",
  "password": "example"
}
```

3. Фотограф создаёт альбом и получает album_id
POST http://127.0.0.1:3000/album
```json
{
  "photographer_id": 1,
  "name": "choose now"
}
```

4. Фотограф загружает все фото, для каждого запрос:
POST http://127.0.0.1:3000/upload
`album_id` = `1`
`file_name` = `photo_1`
`file` = ...

5. Фотограф по email получает id пользователя
GET http://127.0.0.1:3000/user?email=client@client.com

6. Фотограф создаёт приглашение
POST http://127.0.0.1:3000/invitation
```json
{
  "client_id": 2,
  "album_id": 1,
  "photographer_id": 1
}
```

7. Клиент получает информацию о доступных ему альбомах (album_id)
GET http://127.0.0.1:3000/albums/invited?user_id=2

8. Клиент получает фотографии из альбома
GET http://127.0.0.1:3000/photos?album_id=1

9. Клиент выбирает фотографии
POST http://127.0.0.1:3000/selections
```json
{
  
  "album_id": 1,
  "client_id": 2,
  "photo_ids": [1, 2]
}
```

10. Фотограф получает информацию о выбранных фотографиях
GET http://127.0.0.1:3000/selected_photo?client_id=2&album_id=1
