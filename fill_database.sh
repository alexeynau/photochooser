
BACKEND_URL="http://localhost:3000"

# Create a some users
for i in {1..10}
do
  set -e
  curl -X POST $BACKEND_URL/sign_up \
  -H "Content-Type: application/json" \
  -d '{"username":"User'$i'", "email":"user_'$i'@email.com", "password":"example"}' 
done

# Create some albums
for i in {1..5}
do
  set -e
  curl -X POST $BACKEND_URL/album \
  -H "Content-Type: application/json" \
  -d '{"name":"Album'$i'",  "photographer_id":'$i'}' 
done