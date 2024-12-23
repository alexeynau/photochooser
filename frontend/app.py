from io import BytesIO
import streamlit as st
import requests
import json
from PIL import Image
from models import Album, User
import zipfile

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
        user = User.from_json(json.dumps(user_data))
        st.session_state.user = user
        st.session_state.user_id = user_data.get("user_id")
        st.write("Response:", user_data)
        # redirect to create album page
        st.switch_page(st.Page(album_page, title="Album Page"))


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
        response = requests.get(f"{BASE_URL}/photo", params={"photo_id": photo['photo_id']})
        if response.status_code == 200:
            image = Image.open(BytesIO(response.content))
        else:
            st.error(f"Error fetching image. {response.status_code}: {response.text}")
            continue
        st.image(image, caption=photo['s3_path'])
        if st.checkbox(f"Select Photo {photo['photo_id']}", key=f"select_photo_{photo['photo_id']}"):
            selected_photos.append(photo['photo_id'])
    if st.button("Save Selection"):
        response = requests.post(f"{BASE_URL}/selections", json={
            "album_id": st.session_state.selected_album_id,
            "client_id": st.session_state.user_id,
            "photo_ids": selected_photos
        })
        st.write("Selection Response:", response.json())


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
    
    with st.form(key='create_album_form'):
        album_name = st.text_input("Album Name")
        submit_button = st.form_submit_button(label='Create Album')
        if submit_button:
            response = requests.post(f"{BASE_URL}/album", json={
                "photographer_id": st.session_state.user_id,
                "name": album_name
            })
            if response.status_code != 200:
                st.error(f"Error creating album. {response.status_code}: {response.text}")
            else:
                album = response.json()
                st.write("Success")
                # Refresh the list of albums
                response = requests.get(f"{BASE_URL}/albums/created", params={"photographer_id": st.session_state.user_id})
                if response.status_code != 200:
                    st.error(f"Error fetching albums. {response.status_code}: {response.text}")
                else:
                    albums = response.json()
                    album_list = [Album.from_json(json.dumps(album)) for album in albums]
        
    
    for album in album_list:
        with st.expander(f"Album: {album.name}"):
            st.write(f"Created At: {album.created_at}")
            if st.button("Upload Photos", key=f"upload_photos_{album.album_id}"):
                st.session_state.album_id = album.album_id
                st.switch_page(st.Page(upload_photos_page, title="Upload Photos to {}".format(album.name)))
            if st.button("Invite Clients", key=f"invite_clients_{album.album_id}"):
                st.session_state.album_id = album.album_id
                with st.form(key='invite_clients_form'):
                    client_email = st.text_input("Client Email")
                    submit_button = st.form_submit_button(label='Invite Client')
                    if submit_button:
                        response = requests.get(f"{BASE_URL}/user", params={"email": client_email})
                        client_info = response.json()
                        client_id = client_info.get("id")
                        invitation_response = requests.post(f"{BASE_URL}/invitation", json={
                            "client_id": client_id,
                            "album_id": album.album_id,
                            "photographer_id": st.session_state.user_id
                        })
                        if invitation_response.status_code != 200:
                            st.error(f"Error creating invitation. {invitation_response.status_code}: {invitation_response.text}")
                        else:
                            st.write("Invitation Response:", invitation_response.json())
            # download selected photos

            zip_buffer = BytesIO()
            success = False
            with st.form(key='client_email_form_{}'.format(album.album_id)):
                client_email = st.text_input("Client Email")
                submit_button = st.form_submit_button(label='Search Selected Photos')
                if submit_button:
                    response = requests.get(f"{BASE_URL}/user", params={"email": client_email})
                    client_info = response.json()
                    if response.status_code != 200:
                        st.error(f"Error fetching user. {response.status_code}: {response.text}")
                        return
                    client_id = client_info.get("user_id")
                    
                    response = requests.get(f"{BASE_URL}/selected_photo", params={
                        "client_id": client_id,
                        "album_id": album.album_id
                    })
                    if response.status_code == 404:
                        st.error("No photos selected yet. Return later.") 
                        return
                    if response.status_code != 200:
                        st.error(f"Error fetching selected photos. {response.status_code}: {response.text}")
                        return
                    selected_photos = response.json()
                    print("Got {} selected photos".format(len(selected_photos)))
                    
                    # Create a zip file in memory
                    with zipfile.ZipFile(zip_buffer, "a", zipfile.ZIP_DEFLATED, False) as zip_file:
                        for photo in selected_photos:
                            response = requests.get(f"{BASE_URL}/photo", params={"photo_id": photo['photo_id']})
                            if response.status_code == 200:
                                zip_file.writestr(photo['s3_path'], response.content)
                            else:
                                st.error(f"Error downloading image. {response.status_code}: {response.text}")
                    
                    # Ensure the buffer is ready for reading
                    zip_buffer.seek(0)
                    
                    success = True

            if success:
                # Provide the zip file for download
                st.download_button(
                    label=f"Download Selected Photos ({len(selected_photos)})",
                    data=zip_buffer,
                    file_name="selected_photos.zip",
                    mime="application/zip"
                )
            
       
    st.header("Invited Albums")
    response = requests.get(f"{BASE_URL}/albums/invited", params={"client_id": st.session_state.user_id})
    if response.status_code != 200:
        st.error(f"Error fetching albums. {response.status_code}: {response.text}")
        return
    albums = response.json()
    album_list = [Album.from_json(json.dumps(album)) for album in albums]
    for album in album_list:
        with st.expander(f"Album: {album.name}"):
            st.write(f"Created At: {album.created_at}")
            if st.button("View Photos", key=f"view_photos_{album.album_id}"):
                st.session_state.selected_album_id = album.album_id
                st.switch_page(st.Page(view_photos_page, title="View Photos"))
                
            
def check_login():
    if "user_id" not in st.session_state:
        st.error("Please login first.")
        return False
    return True

def main():

    pg = st.navigation(
        [
            st.Page(register_page, title="Register"),
            st.Page(login_page, title="Login"),
            st.Page(album_page, title="Album Page"),
            st.Page(upload_photos_page, title="Upload Photos"),
            st.Page(view_photos_page, title="View Photos")
        ]
    )
    
    pg.run()

if __name__ == "__main__":
    main()



