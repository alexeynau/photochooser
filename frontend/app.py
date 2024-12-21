import streamlit as st
import requests
import json

BASE_URL = "http://127.0.0.1:3000"

def register_page():
    st.title("Register")
    username = st.text_input("Username")
    email = st.text_input("Email")
    password = st.text_input("Password", type="password")
    if st.button("Register"):
        response = requests.post(f"{BASE_URL}/sign_up", json={
            "username": username,
            "email": email,
            "password": password
        })
        st.write("Response:", response.status_code)
        # redirect to login page
        st.session_state.page = "Login"

def login_page():
    st.title("Login")
    email = st.text_input("Email")
    password = st.text_input("Password", type="password")
    if st.button("Login"):
        response = requests.post(f"{BASE_URL}/login", json={
            "email": email,
            "password": password
        })
        if response.status_code != 200:
            st.error(f"Invalid credentials. {response.text}")
            return
        user_data = response.json()
        st.session_state.user_id = user_data.get("user_id")
        st.write("Response:", user_data)
        # redirect to create album page
        st.session_state.page = "Create Album"

def create_album_page():
    st.title("Create Album")
    album_name = st.text_input("Album Name")
    if st.button("Create Album"):
        response = requests.post(f"{BASE_URL}/album", json={
            "photographer_id": st.session_state.user_id,
            "name": album_name
        })
        if response.status_code != 200:
            st.error(f"Error creating album. {response.status_code}: {response.text}")
            return
        album = response.json()
        st.session_state.album_id = album.get("album_id")
        st.write("Response:", album)

def upload_photos_page():
    st.title("Upload Photos")
    if "album_id" not in st.session_state:
        st.error("Please create an album first.")
        return
    photo_files = st.file_uploader("Upload Photos", accept_multiple_files=True)
    if st.button("Upload"):
        for photo in photo_files:
            files = {"file": photo.getvalue()}
            data = {"album_id": st.session_state.album_id, "file_name": photo.name}
            response = requests.post(f"{BASE_URL}/upload", files=files, data=data)
            st.write(f"Uploaded {photo.name}:", response.json())
    if st.button("Create Invitation"):
        client_email = st.text_input("Client Email")
        response = requests.get(f"{BASE_URL}/user", params={"email": client_email})
        client_info = response.json()
        client_id = client_info.get("id")
        invitation_response = requests.post(f"{BASE_URL}/invitation", json={
            "client_id": client_id,
            "album_id": st.session_state.album_id,
            "photographer_id": st.session_state.user_id
        })
        if invitation_response.status_code != 200:
            st.error(f"Error creating invitation. {invitation_response.status_code}: {invitation_response.text}")
            return
        st.write("Invitation Response:", invitation_response.json())

def view_albums_page():
    st.title("View Albums")
    response = requests.get(f"{BASE_URL}/albums/invited", params={"user_id": st.session_state.user_id})
    if response.status_code != 200:
        st.error(f"Error fetching albums. {response.status_code}: {response.text}")
        return
    albums = response.json()
    st.write("Albums:", albums)
    selected_album_id = st.selectbox("Select Album", [album["id"] for album in albums])
    if st.button("View Photos"):
        st.session_state.selected_album_id = selected_album_id
        st.session_state.page = "view_photos"

def view_photos_page():
    st.title("View Photos")
    if "selected_album_id" not in st.session_state:
        st.error("Please select an album first.")
        return
    response = requests.get(f"{BASE_URL}/photos", params={"album_id": st.session_state.selected_album_id})
    if response.status_code != 200:
        st.error(f"Error fetching photos. {response.status_code}: {response.text}")
        return
    photos = response.json()
    selected_photos = []
    for photo in photos:
        if st.checkbox(f"Select Photo {photo['id']}"):
            selected_photos.append(photo['id'])
    if st.button("Save Selection"):
        response = requests.post(f"{BASE_URL}/selections", json={
            "album_id": st.session_state.selected_album_id,
            "client_id": st.session_state.user_id,
            "photo_ids": selected_photos
        })
        st.write("Selection Response:", response.json())

def download_selected_photos_page():
    st.title("Download Selected Photos")
    response = requests.get(f"{BASE_URL}/selected_photo", params={
        "client_id": st.session_state.user_id,
        "album_id": st.session_state.album_id
    })
    if response.status_code != 200:
        st.error(f"Error fetching selected photos. {response.status_code}: {response.text}")
        return
    selected_photos = response.json()
    st.write("Selected Photos:", selected_photos)

def album_page():
    st.title("Album Page")
    if not check_login():
        return
    
    st.write("Welcome to the album page")
    st.header("Created Albums")
    response = requests.get(f"{BASE_URL}/albums/created", params={"photographer_id": st.session_state.user_id})
    if response.status_code != 200:
        st.error(f"Error fetching albums. {response.status_code}: {response.text}")
        return
    albums = response.json()
    album_list = [Album.from_json(json.dumps(album)) for album in albums]
    for album in album_list:
        st.write(f"Album ID: {album.album_id}, Name: {album.name}, Photographer ID: {album.photographer_id}, Created At: {album.created_at}")
        
    st.header("Invited Albums")
    response = requests.get(f"{BASE_URL}/albums/invited", params={"client_id": st.session_state.user_id})
    if response.status_code != 200:
        st.error(f"Error fetching albums. {response.status_code}: {response.text}")
        return
    albums = response.json()
    album_list = [Album.from_json(json.dumps(album)) for album in albums]
    for album in album_list:
        st.write(f"Album ID: {album.album_id}, Name: {album.name}, Photographer ID: {album.photographer_id}, Created At: {album.created_at}")
        
def check_login():
    if "user_id" not in st.session_state:
        st.error("Please login first.")
        return False
    return True

def main():
    pages = {
        "Register": register_page,
        "Login": login_page,
        "Album Page": album_page,
        "Create Album": create_album_page,
        "Upload Photos": upload_photos_page,
        "View Albums": view_albums_page,
        "View Photos": view_photos_page,
        "Download Selected Photos": download_selected_photos_page
    }

    if "page" not in st.session_state:
        st.session_state.page = "Register"

    st.sidebar.title("Navigation")
    page = st.sidebar.selectbox("Go to", list(pages.keys()))
    st.session_state.page = page

    pages[st.session_state.page]()

if __name__ == "__main__":
    main()



# models
class Album:
    def __init__(self, id, name, photographer_id, created_at=None):
        self.album_id = id
        self.name = name
        self.photographer_id = photographer_id
        self.created_at = created_at

    def to_json(self):
        return json.dumps(self, default=lambda o: o.__dict__)

    @staticmethod
    def from_json(json_str):
        data = json.loads(json_str)
        return Album(**data)