import json
# models
class Album:
    def __init__(self, album_id, name, photographer_id, created_at=None):
        self.album_id = album_id
        self.name = name
        self.photographer_id = photographer_id
        self.created_at = created_at

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__)

    @staticmethod
    def from_json(json_str):
        data = json.loads(json_str)
        return Album(**data)
    
class User:
    def __init__(self, user_id, username, email, created_at=None):
        self.user_id = user_id
        self.username = username
        self.email = email
        self.created_at = created_at

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__)

    @staticmethod
    def from_json(json_str):
        data = json.loads(json_str)
        return User(**data)