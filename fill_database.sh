
BACKEND_URL="http://localhost:3000"

# Создать пользователей
for i in {1..10}
do
  set -e
  curl -X POST $BACKEND_URL/sign_up \
  -H "Content-Type: application/json" \
  -d '{"username":"User'$i'", "email":"user_'$i'@email.com", "password":"example"}' 
done

# Создать альбом
for i in {1..5}
do
  set -e
  curl -X POST $BACKEND_URL/album \
  -H "Content-Type: application/json" \
  -d '{"name":"Album'$i'",  "photographer_id":'$i'}' 
done

# Загрузить фотографии
for i in {1..5}
do
  for file in /photos/*; do
  set -e
  curl -X POST $BACKEND_URL/upload \
    --form 'album_id='$i'' \
    --form 'file_name="photo_3"' \
    --form 'file=@"'file'"'
done




